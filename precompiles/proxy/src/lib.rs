// Copyright 2019-2022 PureStake Inc.
// This file is 	part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(assert_matches)]

use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
use pallet_evm::AddressMapping;
use pallet_proxy::Call as ProxyCall;
use pallet_proxy::Pallet as ProxyPallet;
use precompile_utils::data::Address;
use precompile_utils::prelude::*;
use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	AddProxy = "addProxy(address,uint8,uint32)",
	RemoveProxy = "removeProxy(address,uint8,uint32)",
	RemoveProxies = "removeProxies()",
	IsProxy = "isProxy(address,uint8)",
}

/// A precompile to wrap the functionality from pallet-proxy.
pub struct ProxyWrapper<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for ProxyWrapper<Runtime>
where
	Runtime: pallet_proxy::Config + pallet_evm::Config + frame_system::Config,
	<<Runtime as pallet_proxy::Config>::Call as Dispatchable>::Origin:
		From<Option<Runtime::AccountId>>,
	<Runtime as pallet_proxy::Config>::ProxyType: TryFrom<u8>,
	<Runtime as frame_system::Config>::Call:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<Runtime as frame_system::Config>::Call as Dispatchable>::Origin:
		From<Option<Runtime::AccountId>>,
	<Runtime as frame_system::Config>::Call: From<ProxyCall<Runtime>>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;
		handle.check_function_modifier(FunctionModifier::NonPayable)?;

		match selector {
			Action::AddProxy => Self::add_proxy(handle),
			Action::RemoveProxy => Self::remove_proxy(handle),
			Action::RemoveProxies => Self::remove_proxies(handle),
			Action::IsProxy => Self::is_proxy(handle),
		}
	}
}

impl<Runtime> ProxyWrapper<Runtime>
where
	Runtime: pallet_proxy::Config + pallet_evm::Config + frame_system::Config,
	<<Runtime as pallet_proxy::Config>::Call as Dispatchable>::Origin:
		From<Option<Runtime::AccountId>>,
	<Runtime as pallet_proxy::Config>::ProxyType: TryFrom<u8>,
	<Runtime as frame_system::Config>::Call:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<Runtime as frame_system::Config>::Call as Dispatchable>::Origin:
		From<Option<Runtime::AccountId>>,
	<Runtime as frame_system::Config>::Call: From<ProxyCall<Runtime>>,
{
	/// Register a proxy account for the sender that is able to make calls on its behalf.
	/// The dispatch origin for this call must be Signed.
	///
	/// Parameters:
	/// * delegate: The account that the caller would like to make a proxy.
	/// * proxy_type: The permissions allowed for this proxy account.
	/// * delay: The announcement period required of the initial proxy. Will generally be zero.
	fn add_proxy(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;

		let delegate: H160 = input.read::<Address>()?.into();
		let proxy_type = input
			.read::<u8>()?
			.try_into()
			.map_err(|_| revert("failed decoding proxy_type"))?;
		let delay = input.read::<u32>()?.into();

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = ProxyCall::<Runtime>::add_proxy {
			delegate: Runtime::AddressMapping::into_account_id(delegate),
			proxy_type,
			delay,
		}
		.into();

		<RuntimeHelper<Runtime>>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed([]))
	}

	/// Unregister a proxy account for the sender.
	/// The dispatch origin for this call must be Signed.
	///
	/// Parameters:
	/// * delegate: The account that the caller would like to remove as a proxy.
	/// * proxy_type: The permissions currently enabled for the removed proxy account.
	/// * delay: The announcement period required of the initial proxy. Will generally be zero.
	fn remove_proxy(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;

		let delegate: H160 = input.read::<Address>()?.into();
		let proxy_type = input
			.read::<u8>()?
			.try_into()
			.map_err(|_| revert("failed decoding proxy_type"))?;
		let delay = input.read::<u32>()?.into();

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = ProxyCall::<Runtime>::remove_proxy {
			delegate: Runtime::AddressMapping::into_account_id(delegate),
			proxy_type,
			delay,
		}
		.into();

		<RuntimeHelper<Runtime>>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed([]))
	}

	/// Unregister all proxy accounts for the sender.
	/// The dispatch origin for this call must be Signed.
	/// WARNING: This may be called on accounts created by anonymous, however if done, then the
	/// unreserved fees will be inaccessible. All access to this account will be lost.
	fn remove_proxies(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = ProxyCall::<Runtime>::remove_proxies {}.into();

		<RuntimeHelper<Runtime>>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed([]))
	}

	/// Checks if the caller is a proxy account for the real account with a given proxy type
	///
	/// Parameters:
	/// * real: The real account that the caller is maybe a proxy for
	/// * proxyType: The permissions allowed for the proxy caller
	fn is_proxy(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let real: H160 = input.read::<Address>()?.into();
		let real = Runtime::AddressMapping::into_account_id(real);
		let proxy_type: Runtime::ProxyType = input
			.read::<u8>()?
			.try_into()
			.map_err(|_| revert("failed decoding proxy_type"))?;

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let is_proxy = ProxyPallet::<Runtime>::proxies(real)
			.0
			.iter()
			.any(|pd| pd.delegate == origin && pd.proxy_type == proxy_type);

		Ok(succeed(EvmDataWriter::new().write(is_proxy).build()))
	}
}
