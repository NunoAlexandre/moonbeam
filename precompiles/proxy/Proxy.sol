// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/// @author The Moonbeam Team
/// @title The interface through which solidity contracts will interact with the Proxy pallet
/// @custom:address 0x000000000000000000000000000000000000080b
interface Proxy {
    /// @dev Defines the proxy permission types that may be combined via `|` operator
    /// The values start at `0` and are represented as `uint32`
    enum ProxyType {
        Any,
        NonTransfer,
        Governance,
        Staking,
        CancelProxy,
        Balances,
        AuthorMapping,
        IdentityJudgement
    }

    /// @dev Register a proxy account for the sender that is able to make calls on its behalf
    /// @custom:selector ac69400b
    /// @param delegate the account that the caller would like to make a proxy
    /// @param proxyType the permissions allowed for this proxy account
    /// @param delay the announcement period required of the initial proxy, will generally be zero
    function addProxy(
        address delegate,
        ProxyType proxyType,
        uint32 delay
    ) external;

    /// @dev Register a proxy account for the sender that is able to make calls on its behalf
    /// @custom:selector 78a804c5
    /// @param delegate The account that the caller would like to remove as a proxy
    /// @param proxyType The permissions currently enabled for the removed proxy account
    /// @param delay The announcement period required of the initial proxy, will generally be zero
    function removeProxy(
        address delegate,
        ProxyType proxyType,
        uint32 delay
    ) external;

    /// @dev Unregister all proxy accounts for the sender
    /// @custom:selector 14a5b5fa
    function removeProxies() external;
}
