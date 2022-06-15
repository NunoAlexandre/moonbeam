initSidebarItems({"enum":[["Call","Contains one variant per dispatchable that can be called by an extrinsic."],["Error","An error that can occur while executing the mapping pallet’s logic."],["Event","The event emitted by this pallet."]],"struct":[["Pallet","The pallet implementing the on-chain logic."],["RemoteTransactInfoWithMaxWeight","Stores the information to be able to issue a transact operation in another chain use an asset as fee payer."]],"trait":[["Config","Configuration trait of this pallet."]],"type":[["DestinationAssetFeePerSecond","Stores the fee per second for an asset in its reserve chain. This allows us to convert from weight to fee"],["IndexToAccount","Since we are using pallet-utility for account derivation (through AsDerivative), we need to provide an index for the account derivation. This storage item stores the index assigned for a given local account. These indices are usable as derivative in the relay chain"],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."],["TransactInfoWithWeightLimit","Stores the transact info of a MultiLocation. This defines how much extra weight we need to add when we want to transact in the destination chain and maximum amount of weight allowed by the destination chain"]]});