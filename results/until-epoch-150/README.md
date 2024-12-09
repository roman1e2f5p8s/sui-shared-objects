# Analysis of the most used shared objects in Sui

## Table of Contents

- [Introduction](#introduction)
    - [Most important findings](#most-important-findings)

---

## Introduction
This document contains analysis of the most frequently used **shared object 
types** in the Sui (mainnet) network.

> [!NOTE]
> These results are based on the collected data about Sui programmable 
> transactions starting from epoch 0 until epoch 150, inclusive.

### Most important findings
- In general, shared objects are frequently used in Sui. However, the density of transactions involving shared objects significantly varies over time, from extremely low values during the early stages of the Sui main network and the Bullshark Quest 1 period to high values during the Bullshark Quest 2 period.
- The average degree of contention for the same shared object within a checkpoint interval is less than 1, meaning that contention is very low on average. However, as expected, increasing the duration of intervals significantly increases the average contention degree, which implies that contention for shared objects will be less prominent in fast-committing consensus protocols.
- The fraction of shared objects touched by more than one transaction with respect to the total number of shared objects within some interval is not very high (less than 0.5 on average), which means that a given shared object is more frequently touched by only one transaction rather than multiple ones. Moreover, this fraction does not vary too much with increasing the duration of an interval.
- Two applications, [`Pyth Network`](#1-Pyth-Network-1) and [`Sui Framework`](#2-Sui-Framework), that implement shared objects constitute more than half (53.9%) of all transactions involving shared objects.
- Two shared objects, [`PriceInfoObject`](#-Type-PriceInfoObject) and [`Clock`](#-Type-Clock), are involved in roughly a half (≈46%) of all shared-object transactions. Transactions involving `PriceInfoObject` constitute 29.4% of all shared-object transactions and usually (in ≈97% of all transactions involving `PriceInfoObject`) take `PriceInfoObject` by a mutable reference, i.e., write to it, which likely indicates a high level of contention for `PriceInfoObject` shared objects.
- There are shared object types, instances of which can be updated only by a single writer while they are available for reading by anyone. Examples of such shared objects include [`ProtocolConfigs`](#-Type-ProtocolConfigs), [`Game`](#-Type-Game), [`Versioned`](#-Type-Versioned), [`Version`](#-Type-Version). Moreover, such shared objects are usually accessed via read operations, meaning contention is not high.
- Extensively used [`Clock`](#-Type-Clock) shared object (16.3% of all shared-object transactions) can only be read in programmable transactions. `Clock` can only be updated via a special protocol transaction that can be issued only by the Sui validators. Even though this read-only mode prevents contention, transactions reading `Clock` can not follow a fast path in Sui but must go through consensus.
- Some shared objects have never been updated, and there are no write operations to be performed on them. Such objects resemble immutable ones and include [`RebaseFeeModel`](#-Type-RebaseFeeModel), [`FundingFeeModel`](#-Type-FundingFeeModel), [`ReservingFeeModel`](#-Type-ReservingFeeModel), and [`WrappedPositionConfig`](#-Type-WrappedPositionConfig) in the `ABEx Finance` application.
- There are object types instances of which can be both shared and/or owned/immutable. These include [`TransferPolicy`](#-Type-TransferPolicy), [`TreasuryCap`](#-Type-TreasuryCap), and [`CoinMetadata`](#-Type-CoinMetadata) types in the Sui Framework contract; however, there may be more in other packages.
- A quite ”popular” use case of shared objects in Sui is liquidity pools usually represented by a `Pool` shared object in such contracts as [`Kriya DEX`](#4-Kriya-DEX), [`Cetus`](#6-Cetus-4), [`DeepBook`](#11-DeepBook), [`Turbos Finance`](#9-Turbos-Finance-1), and other.
- There are shared object types for which many instances have been created. These include [`Game`](#-Type-Game) with nearly 3 millions of instances in `DeSuiLabs Coin Flip`, [`Kiosk`](#-Type-Kiosk) with over 184,000 instances in `Sui Framework`, and [`Obligation`](#-Type-Obligation) with over 55,000 instances in the `Scallop` application.

---

## Packages

The list of the most used packages that implement shared objects is sorted by the number of transactions that touched shared objects in descending order. The full list of all packages that implement shared objects can be found [here](https://github.com/roman1e2f5p8s/sui-shared-object-density/blob/main/results/workspace1/packages_data.json).

:::warning
**NOTE:** This list only contains packages that implement shared object types. Packages that implement exclusively owned object types are not considered here.
:::

:::danger
Data about Sui shared objects was collected until epoch 150, inclusive.
:::

---

### 1. :package: [`Pyth Network 1`](https://suiexplorer.com/object/0x00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302?network=mainnet)
- Source code: [Pyth](https://github.com/pyth-network/pyth-crosschain/tree/main/target_chains/sui/contracts)
- Shared object types:
    - **`PriceInfoObject`** in the `price_info` module: 27 instances
    - **`State`** in the `state` module: 1 instance
- The Pyth Network is a data oracle that publishes financial market data to multiple blockchains
- See [overview of Pyth](https://hackmd.io/wiK4bpFZRSCpXhx-7BpKrA) for more details
- Read about [Simple and Meta Oracles on Sui](https://mystenlabs.com/blog/simple-and-meta-oracles-on-sui)

#### :m: Module [`price_info`](https://suiexplorer.com/object/0x00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302?module=price_info&network=mainnet)
 
##### :red_circle: Type `PriceInfoObject`
- [Pyth price feeds](https://docs.pyth.network/documentation/pythnet-price-feeds) on Sui are represented by the `PriceInfoObject` shared object type. Each instance of `PriceInfoObject` is in unique correspondence with each Pyth price feed in the global storage. `PriceInfoObject` is Sui shared object that wraps `PriceInfo`.
- Definition
    ```rust=
    struct PriceInfoObject has store, key {
	    id: UID,
	    price_info: PriceInfo
    }
    ```
- 27 instances: 77,826,890 TXs (in 75,629,251 TXs - by `&mut`)
    - Each instance corresponds to one price feed
    - In the pull price update model, to read a price, the user must first update it

|Write operations|Who can perform|
|-|-|
|Module `pyth`: `update_single_price_feed`|Anyone with `HotPotatoVector<PriceInfo>`|

|Read operations|Who can perform|
|-|-|
|Module `price_info`: `uid_to_inner`, `get_price_info_from_price_info_object`|Anyone|
|Module `pyth`: `get_price`, `get_price_no_older_than`, `get_price_unsafe` |Anyone|
     
- The need for a Sui shared object to represent `PriceInfoObject` (and thus, price feed) stems from the underlying price update model used by Pyth. Pythnet price feeds use a **pull price update model**, where *users are responsible for posting price updates* on-chain whenever needed. Typically, users of Pyth Network prices will submit a single transaction that simultaneously updates the price and uses it in a downstream application.
- In the pull model, updating the on-chain price is a **permissionless** operation: that is, anyone can submit a verified update message to the Pyth contract to update the price. This would not be possible to realize with Sui owned objects. It also seems that even a push price update model could not be realized using only Sui owned objects in their current design. There could be a Sui single owned object representing a price feed and the owner of that object would need to continuously send transactions to update an on-chain price. However, customers would not be able to read price feed from that owned object since only owner could prove the object's  ownership and access the object. Additionally, having price feed as an owned object would mean that the consumers would depend on another party (service provider, namely, the owner of the owned object) for price feed updates.

---

#### :m: Module [`state`](https://suiexplorer.com/object/0x00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302?module=state&network=mainnet)

##### :red_circle: Type `State`-1
- `State` is a singleton shared object that maintains necessary data about the Pyth package
- A single instance of `State` is instantiated and (publicly) shared when Pyth is initialized during the package deployment
- The initialization of the Pyth package can only be called by the owner of the `DeployerCap` --- the capability that will be destroyed once `State` is made shared
- Creating and destroying the `DeployerCap` capability during the initialization ensures that only the deployer can create the shared `State`
- Definition
	```rust=
	struct State has store, key {
		id: UID,
		governance_data_source: DataDefinition,
		stale_price_threshold: u64,
		base_update_fee: u64,
		fee_recipient_address: address,
		last_executed_governance_sequence: u64,
		consumed_vaas: ConsumedVAAs,
		upgrade_cap: UpgradeCap
	}
	```
- 1 instance: 16,696,329 TXs (in 10,931 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `contract_upgrade`: `authorize_upgrade`|Owner of `DecreeReceipt<GovernanceWitness>`|
|Module `contract_upgrade`: `commit_upgrade`|Owner of `UpgradeReceipt`|
|Module `governance`: `execute_governance_instruction`|Owner of `DecreeReceipt<GovernanceWitness>`|
|Module `migrate`: `migrate`|Owner of `DecreeReceipt<GovernanceWitness>`|
|Module `pyth`: `create_price_feeds`|Anyone with `LatestOnly` capability and `vector<VAA>`|

:::info
Verified Action Approvals (VAAs) are the core messaging primitives transferred over the Wormhole network
:::

|Read operations|Who can perform|
|-|-|
|Module `state`: `get_stale_price_threshold_secs`, `get_base_update_fee`, `get_fee_recipient`, `governance_data_source`, `get_last_executed_governance_sequence`, `governance_chain`, `governance_contract`|Anyone|
|Module `state`: `is_valid_data_source`, `is_valid_governance_data_source`, |Anyone with `DataSource`|
|Module `state`: `price_feed_object_exists`|Anyone with `PriceIdentifier`|
|Module `state`: `get_price_info_object_id`|Anyone|
|Module `contract_upgrade`: `authorize_governance`|Anyone|
|Module `pyth`: `create_price_infos_hot_potato`|Anyone with `vector<VAA>`|
|Module `pyth`: `update_single_price_feed`|Anyone with `HotPotatoVector<PriceInfo>`|
|Module `pyth`: `price_feed_exists`|Anyone with `PriceIdentifier`|
|Module `pyth`: `get_price`, `get_stale_price_threshold_secs`, `get_total_update_fee` |Anyone|
|Module `set_data_sources`: `authorize_governance`|Anyone|
|Module `set_fee_recipient`: `authorize_governance`|Anyone|
|Module `set_governance_data_source`: `authorize_governance`|Anyone|
|Module `set_stale_price_threshold`: `authorize_governance`|Anyone|
|Module `set_update_fee`: `authorize_governance`|Anyone|

- Access control for `State` depends on the operation:
    - ++Simple getters++: These read-only methods do not require special access. Anyone is free to access `State` values using these getters.
    - ++Privileged `State` access++: Some methods require the `LatestOnly` capability, which can only be created within the Wormhole package. This capability allows special access to the `State` object.
        - `LatestOnly` is capability that reflects that the current build version is used to invoke state methods.
    - ++Upgradeability++: Special methods that control upgrade logic. These methods are invoked via the [`contract_upgrade`](https://github.com/pyth-network/pyth-crosschain/blob/main/target_chains/sui/contracts/sources/governance/contract_upgrade.move) module.
    - ++Migrate++: A very special method that manipulates `State` via calling `migrate`.
---

### 2. :package: [`Sui Framework`](https://suiexplorer.com/object/0x0000000000000000000000000000000000000000000000000000000000000002?network=mainnet)
- Source code: [sui-framework](https://github.com/MystenLabs/sui/tree/main/crates/sui-framework/packages/sui-framework)
- Shared object types:
    - **`Clock`** in the `clock` module: 1 instance
    - **`Kiosk`** in the `kiosk` module: 125,752 instances
    - **`TransferPolicy`** in the `transfer_policy` module: 155 instances
    - **`TreasuryCap`** in the `coin` module: 15 instances
    - **`CoinMetadata`** in the `coin` module: 177 instances
- [Sui Framework](https://docs.sui.io/sui-framework-reference) includes the core on-chain libraries for Move developers

#### :m: Module [`clock`](https://suiexplorer.com/object/0x2?module=clock&network=mainnet)
:::info
APIs for accessing time from Move calls, via the `Clock`: a unique shared object that is created at `0x6` during genesis
:::

##### :red_circle: Type `Clock`
- `Clock` is a singleton shared object that can only be read when accessing time in Sui
- More info here: [Access On-Chain Time via `Clock`](https://docs.sui.io/guides/developer/sui-101/access-time)
- Definition:
```rust=
/// Singleton shared object that exposes time to Move calls.  This
/// object is found at address 0x6, and can only be read (accessed
/// via an immutable reference) by entry functions.
///
/// Entry Functions that attempt to accept `Clock` by mutable
/// reference or value will fail to verify, and honest validators
/// will not sign or execute transactions that use `Clock` as an
/// input parameter, unless it is passed by immutable reference.
struct Clock has key {
    id: UID,
    /// The clock's timestamp, which is set automatically by a
    /// system transaction every time consensus commits a
    /// schedule, or by `sui::clock::increment_for_testing` during
    /// testing.
    timestamp_ms: u64,
}
```
- 1 instance: 44,596,744 TXs
- Anyone can access `Clock` only by an immutable (read-only) reference
- Example of TX that has only `Clock` shared object in its inputs: [`7hjPyk2USjBjEPEJ2JLiKG4cYq3AmsGfB3VEE94YvBsX`](https://suiexplorer.com/txblock/7hjPyk2USjBjEPEJ2JLiKG4cYq3AmsGfB3VEE94YvBsX?network=mainnet))
- Checkpoint timestamps are monotonic, but not strongly: subsequent checkpoints can have same timestamp if they originate from the same underlining consensus commit.
- `Clock` timestamp is expected to change every 2-3 seconds, at the rate the network commits checkpoints
- Successive calls to `sui::clock::timestamp_ms` in the same TX always produce the same result (TXs are considered to take effect instantly), but timestamps from `Clock` are otherwise monotonic across TXs that touch the same shared objects: successive TXs seeing a greater or equal timestamp to their predecessors
- Any TX that requires access to a `Clock` must go through consensus because the only available instance is a shared object. As a result, this technique is not suitable for TXs that must use the single-owner fast path (epoch timestamps are recommended to use for a single-owner-compatible source of timestamps)
- TXs that use `Clock` must accept it as an immutable reference (not a mutable reference or value). This prevents contention, as TXs that access `Clock` can only read it, so do not need to be sequenced relative to each other. Validators refuse to sign transactions that do not meet this requirement and packages that include entry functions that accept a `Clock` or `&mut Clock` fail to publish
- To update `Clock`, validator will make a special system call/TX with sender set as `0x0`. Such a TX of kind `TransactionKind::ConsensusCommitPrologue` is a system TX, so only validators can directly submit it. The `timestamp_ms` field will be modified
- From [consensus.rs](https://github.com/MystenLabs/sui/blob/main/narwhal/types/src/consensus.rs):
   - ```rust
     /// The timestamp that should identify this commit. This is guaranteed to be monotonically
     /// incremented. This is not necessarily the leader's timestamp. We compare the leader's timestamp
     /// with the previously committed sud dag timestamp and we always keep the max.
     /// Property is explicitly private so the method commit_timestamp() should be used instead which
     /// bears additional resolution logic.
     commit_timestamp: TimestampMs,
     ```

|Read operations|Who can perform|
|-|-|
|Get the current timestamp in milliseconds: `timestamp_ms`|Anyone|
- Since `Clock` is a shared object that can only be read by users, it might be considered as a "versioned immutable object" from the point of the user. Sui, however, could not make this object immutable because the `timestamp_ms` field is updated by validators, and there is no other object types except shared, owned, and immutable objects to represent the clock in Sui
- Nevertheless, because `Clock` is a shared object, it seems that transactions that touch only this shared object (and possibly owned objects) must go through consensus in Sui
- It is worth noting that all TXs in the same checkpoint have the same timestamp, so they do not need to be sequenced if they touch only the `Clock` shared object (and possible owned objects)
---

#### :m: Module [`kiosk`](https://suiexplorer.com/object/0x2?module=kiosk&network=mainnet)
:::info
Kiosk is a primitive for building safe, decentralized and trustless trading experiences. It allows storing and trading any types of assets as long as the creator of these assets implements a `TransferPolicy` for them.
:::

##### :red_circle: Type `Kiosk`
- `Kiosk` provides guarantees of "true ownership". Similarly to owned objects, assets stored in `Kiosk` can only be managed by the kiosk owner. Only the owner can `place`, `take`, `list` items, perform any other actions on assets in `Kiosk`
- A third party can purchase items. Every purchase creates `TransferRequest` which must be resolved in a matching `TransferPolicy` for the transaction to pass. This way, the kiosk owner is fully in control of the trading experience
- Anyone can create a Sui `Kiosk`. Ownership of a kiosk is determined by the owner of the `KioskOwnerCap`, a special object that grants full access to a single kiosk. As the owner, you can:
    - sell any asset with a type (`T`) that has a shared `TransferPolicy` available, or 
    - use a kiosk to store assets even without a shared policy.
- You can’t sell or transfer any assets from your kiosk that do not have an associated transfer policy available
- By default, Sui's `Kiosk` is a shared object that can store heterogeneous values, such as different sets of asset collectibles
- See [Sui Kiosk | Sui Documentation](https://docs.sui.io/standards/kiosk) for more details
- Definition:
	```rust=
	/// An object which allows selling collectibles within "kiosk" ecosystem.
	/// By default gives the functionality to list an item openly - for anyone
	/// to purchase providing the guarantees for creators that every transfer
	/// needs to be approved via the `TransferPolicy`.
	struct Kiosk has key, store {
	    id: UID,
	    /// Balance of the Kiosk - all profits from sales go here.
	    profits: Balance<SUI>,
	    /// Always point to `sender` of the transaction.
	    /// Can be changed by calling `set_owner` with Cap.
	    owner: address,
	    /// Number of items stored in a Kiosk. Used to allow unpacking
	    /// an empty Kiosk if it was wrapped or has a single owner.
	    item_count: u32,
	    /// [DEPRECATED] Please, don't use the `allow_extensions` and the matching
	    /// `set_allow_extensions` function - it is a legacy feature that is being
	    /// replaced by the `kiosk_extension` module and its Extensions API.
	    ///
	    /// Exposes `uid_mut` publicly when set to `true`, set to `false` by default.
	    allow_extensions: bool
	}
	```
- 184,509 instances: 4,581,703 TXs (in 1,060,302 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Set a new owner, withdraw profits from `Kiosk`|Owner|
|Place, lock, take any item/object in `Kiosk`|Owner|
|List the item by setting a price and making it available for purchase|Owner|
|Delist: remove an existing listing from the `Kiosk` and keep the item in the user `Kiosk`|Owner|
|Create a `PurchaseCap` which gives the right to purchase an item for any price equal or higher than the `min_price`, and list that item|Owner|
|Allow or disallow extensions, add an extension to the `Kiosk`. A Kiosk Extension is a module that implements any functionality on top of the `Kiosk` without discarding nor blocking the base|Owner|
|Disable extension: revoke permissions from the extension. While it does not remove the extension completely, it keeps it from performing any protected actions|Owner|
|Re-enable the extension allowing it to call protected actions, remove an extension from `Kiosk`|Owner|
|Immutably and mutably borrow any item from the `Kiosk` at any time. Item can be mutably borrowed only if it's not `is_listed`|Owner|
|Borrow by value: take the item from the `Kiosk` with a guarantee that it will be returned. Item can be borrowed by value only if it's not `is_listed`|Owner|
|Return the borrowed item to the `Kiosk`. This operation cannot be avoided if `borrow_val` is used|Owner|
|Access the `id` field as a `&mut UID` using the `KioskOwnerCap`, get mutable access to `profits` of the `Kiosk`|Owner|
|
|Purchase item from `Kiosk`: pay the owner of the item and request a transfer to the `target` (purchaser's) kiosk (to prevent item being taken by the approving party). This involves removing dynamic (object) fields (i.e., item), decrementing item count, and putting payment into the `Kisok` profits|Anyone|
|Purchase item with `PurchaseCap`: set the payment amount as the price for the listing making sure it's not less than `min_amount`|Anyone|
|Return the `PurchaseCap` without making a purchase: remove an active offer and allow the item for taking. Can only be returned to its `Kiosk`, aborts otherwise|Anyone|
|Check whether the `KioskOwnerCap` matches `Kiosk`, get the `id` field as a `&mut UID` for dynamic field access and extensions|Anyone|
|Get mutable access to the extension storage. Can only be performed by the extension as long as the extension is installed|Anyone|
|Protected action: place an item into the Kiosk. Can be performed by an authorized extension. The extension must have the `place` permission or a `lock` permission|Anyone|
|Protected action: lock an item in the Kiosk. Can be performed by an authorized extension. The extension must have the `lock` permission|Anyone|

|Create/destroy operations|Who can perform|
|-|-|
|Delete `Kiosk` (if it is not shared and does not have items)|Owner|

|Read operations|Who can perform|
|-|-|
|Check whether the item (of type `T`) is present in the `Kiosk`, where the item is locked or listed|Anyone|
|Check whether there's a `PurchaseCap` issued for an item|Anyone|
|Get an immutable `&UID` for dynamic field access|Anyone|
|Get the owner, the number of items stored, the amount of profits collected by selling items in the `Kiosk`|Anyone|
|Get immutable access to the extension storage. Can only be performed by the extension as long as the extension is installed|Anyone|
|Check whether an extension of type `Ext` is installed, enabled, can `place` or `lock` items in `Kiosk`|Anyone|
- `Kiosk` resembles an owned object. Ownership access control is defined by the `KioskOwnerCap` capability, which grants the bearer a right to `place` and `take` items from the `Kiosk` as well as to `list` them and `list_with_purchase_cap`.
- The sender of the transaction creates a `Kiosk` object and becomes its owner (`KioskOwnerCap` is transferred to the sender/owner).
    - By default, the `Kiosk` object is made shared during creation via `share_object`.
- Items in `Kiosk` are stored as dynamic fields and are freely available to anyone for purchase. Purchasing an item from `Kiosk` is a write operation. This seems to be the main reason why `Kiosk` objects are made shared.
- For more advanced use cases, when you want to choose the storage model or perform an action right away, you can use the programmable transaction block (PTB) friendly function `kiosk::new`. `Kiosk` is designed to be shared. If you choose a different storage model, such as owned, your kiosk might not function as intended or not be accessible to other users.

---

#### :m: Module [`transfer_policy`](https://suiexplorer.com/object/0x2?module=transfer_policy&network=mainnet)

##### :red_circle: Type `TransferPolicy`
- `TransferPolicy` is a highly customizable primitive, which provides an interface for the type owner to set custom transfer rules for every deal performed in the `Kiosk` or a similar system that integrates with `TransferPolicy`.
- Type owner (creator) can set any rules as long as the ecosystem supports them.
- Once `TransferPolicy<T>` is created and shared (or frozen), the type `T` becomes tradable in `Kiosk`. Trading any types of assets is not possible unless the creator of these assets implements `TransferPolicy` for them.
- On every purchase operation, `TransferRequest` is created and needs to be confirmed by `TransferPolicy`, otherwise transaction will fail.
- `TransferPolicy` aims to be the main interface for creators to control trades of their types and collect profits if a fee is required on sales. Custom policies can be removed at any moment, and the change will affect all instances of the type at once.
- To sell an item, if there is an existing transfer policy for the type (`T`), you just add your assets to your kiosk and then list them. You specify an offer amount when you list an item. Anyone can then purchase the item for the amount of SUI specified in the listing. The associated transfer policy determines what the buyer can do with the purchased asset.
- Definition:
    ```rust=
    /// A unique capability that allows the owner of the `T` to authorize
    /// transfers. Can only be created with the `Publisher` object. Although
    /// there's no limitation to how many policies can be created, for most
    /// of the cases there's no need to create more than one since any of the
    /// policies can be used to confirm the `TransferRequest`.
    struct TransferPolicy<phantom T> has key, store {
        id: UID,
        /// The Balance of the `TransferPolicy` which collects `SUI`.
        /// By default, transfer policy does not collect anything , and it's
        /// a matter of an implementation of a specific rule - whether to add
        /// to balance and how much.
        balance: Balance<SUI>,
        /// Set of types of attached rules - used to verify `receipts` when
        /// a `TransferRequest` is received in `confirm_request` function.
        ///
        /// Additionally provides a way to look up currently attached Rules.
        rules: VecSet<TypeName>
    }
    ```
- 174 shared instances: 691,784 TXs (in 448,988 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Withdraw some amount of profits from the `TransferPolicy`|Owner|
|Add or remove a custom rule to the `TransferPolicy`|Owner|
|Get a mutable reference to the ID (`&mut UID`) to enable custom attachments to the `TransferPolicy`|Owner|
|
|Add some `SUI` to the balance of a `TransferPolicy` if such rule has been set |Anyone|

|Create/destroy operations|Who can perform|
|-|-|
|Destroy `TransferPolicy` along with `TransferPolicyCap` and withdraw profits|Owner|

|Read operations|Who can perform|
|-|-|
|Confirm request: allow a `TransferRequest` for the type `T`.|Owner|
|
|Get the custom config for the rule|Anyone|
|Check whether a custom rule has been added to the `TransferPolicy`|Anyone|
|Get a reference to the ID (`&UID`): allows reading custom attachments to the `TransferPolicy` if there are any| Anyone |
|Read the `rules` field from the `TransferPolicy`|Anyone|
- `TransferPolicy` resembles an owned object. Ownership access control is defined by the `TransferPolicyCap` capability, which grants the owner permission to add/remove rules as well as to `withdraw` and `destroy_and_withdraw` the `TransferPolicy`.
- In the default scenario, `TransferPolicy` is shared (inside its module, i.e., via `share_object`) and `TransferPolicyCap` is transferred to the transaction sender.
- Only publisher of the package can create and destroy `TransferPolicy`.
    - Once created, `TransferPolicy` does not necessary need to become shared, it can be frozen as well.
    - Example of an immutable object: [`TransferPolicy<RedBullCollectible>`](https://suiexplorer.com/object/0xd0ecf58bdc29ce5f4834bc32f1e9d322c7e86eed167a8b779c66bfd4ac2aa0c7?network=mainnet).
- It probably depends on type `T` whether the creator needs to make `TransferPolicy` shared or not.

---

#### :m: Module [`coin`](https://suiexplorer.com/object/0x2?module=coin&network=mainnet)
:::info
Defines the `Coin` type - platform wide representation of fungible tokens and coins. `Coin` can be described as a secure wrapper around `Balance` type.
:::

##### :red_circle: Type `TreasuryCap`
- `TreasuryCap` provides the capability that allows the bearer to mint and burn coins of some type `T`. It acts as a wrapper around `Supply` and can be transferred.
- `TreasuryCap` guarantees full ownership over the currency, and is unique, hence it is safe to use it for authorization.
- `TreasuryCap` can be created only once for a type. When registering a new coin type `T`, the `TreasuryCap` for `T` will be created and returned to the caller. This can be done only once to ensure that there's only one `TreasuryCap` per `T`.
- Definition:
    ```rust=
    // Capability allowing the bearer to mint and burn
    /// coins of type `T`. Transferable
    struct TreasuryCap<phantom T> has key, store {
        id: UID,
        total_supply: Supply<T>
    }
    ```
- 19 instances: 4,671 TXs (always passed by `&mut`)
 
|Write operations|Who can perform|
|-|-|
|Mint coin/token: create a coin. The total supply `TreasuryCap` will be increased accordingly. The coin/token will be transferred to the recipient|Owner*|
|Mint balance (some amount of `T`). The total supply `TreasuryCap` will be increased accordingly|Owner*|
|Destroy a coin/burn a token. The total supply `TreasuryCap` will be decreased accordingly|Owner*|
|Get mutable reference to the `total_supply` of `TreasuryCap`|Owner*|
|Confirm an `ActionRequest` as the `TreasuryCap` owner. This allows `spent_balance` of `TokenPolicy` to be accessed. The `total_supply` of the `Token` will be decreased|Owner|
|Flush the `TokenPolicy.spent_balance` into the `TreasuryCap`|Owner|

|Create/destroy operations|Who can perform|
|-|-|
|Consume `TreasuryCap` by converting it into the total supply. The `TreasuryCap` object will be deleted. This operation is irreversible: supply cannot be converted into a `TreasuryCap` due to different security guarantees (`TreasuryCap` can be created only once for a type)|Owner* |

|Read operations|Who can perform|
|-|-|
|Create a new `TokenPolicy` and a matching `TokenPolicyCap`. `TreasuryCap` guarantees full ownership over the currency, and is unique, hence it is safe to use it for authorization|Owner*|
|||
|Get the total number (supply) of coins of some type `T`' in circulation|Anyone*|
|Get immutable reference to the `total_supply` of `TreasuryCap`|Anyone*|
|Update coin metadata `CoinMetadata` (name, description, url)|Anyone*|

*\* likely but not sure (couldn't find it)*
- `TreasuryCap` resembles an owned object.
    - In [Managed coin example](https://github.com/sui-foundation/sui-move-intro-course/blob/main/unit-three/lessons/5_managed_coin.md), `TreasuryCap` is an owned object.
    - In [Create coin example](https://examples.sui.io/samples/coin.html), `TreasuryCap` is an owned object.
    - Example of an owned object on Sui Explorer: [TreasuryCap<0x6daf…ffe4::tusdc::TUSDC>](https://suiexplorer.com/object/0x127f37ea2442c695086c6357f9802e45a09dececc3f2d0e05feefb721e0de167?network=mainnet)
- `TreasuryCap` is made shared, for example, in the [apc](https://suiexplorer.com/object/0x62fff363d8919c16c3354e452800afc92cc1815296475bb87b769b5e6468dc38?module=apc&network=mainnet) module.
- It probably depends on type `T` whether the creator needs to make `TreasuryCap` shared or not.

---

##### :red_circle: Type `CoinMetadata`
- Each Coin type `T` created via `create_currency` will have a unique instance of `CoinMetadata<T>` that stores the metadata for this coin type.
- Definition:
```rust=
/// Each Coin type T created through `create_currency` function will have a
/// unique instance of CoinMetadata<T> that stores the metadata for this coin type.
struct CoinMetadata<phantom T> has key, store {
    id: UID,
    /// Number of decimal places the coin uses.
    /// A coin with `value ` N and `decimals` D should be shown as N / 10^D
    /// E.g., a coin with `value` 7002 and decimals 3 should be displayed as 7.002
    /// This is metadata for display usage only.
    decimals: u8,
    /// Name for the token
    name: string::String,
    /// Symbol for the token
    symbol: ascii::String,
    /// Description of the token
    description: string::String,
    /// URL for the token logo
    icon_url: Option<Url>
}
```
- 198 instances: 397 TXs (in 190 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Update the name, symbol, description, url of the coin in `CoinMetadata`|Owner*|

*\* likely but not sure (couldn't find it)*

|Read operations|Who can perform|
|-|-|
|Get coin metadata fields for on-chain consumption: decimals, name, symbol, description, icon url|Anyone|

- `CoinMetadata` resembles an owned object. Owned object of this type might exist.
    - Example of an immutable object: [`CoinMetadata<0x2::sui::SUI>`](https://suiexplorer.com/object/0x9258181f5ceac8dbffb7030890243caed69a9599d2886d957a9cb7656af3bdb3?module=coin&network=mainnet)
- Could not find where `CoinMetadata` is made shared.
- It probably depends on type `T` whether the creator needs to make `CoinMetadata` shared or not.

---

#### :m: Module [`table`](https://suiexplorer.com/object/0x2?module=table&network=mainnet)
:::info
A table is a map-like collection. But unlike a traditional collection, it's keys and values are not stored within the `Table` value, but instead are stored using Sui's object system. The `Table` struct acts only as a handle into the object system to retrieve those keys and values.
:::

##### :red_circle: Type `Table`
- `Table` is a collection built using dynamic fields, but with additional support to count the number of entries they contain, and protect against accidental deletion when non-empty.
- `Table<K, V>` is a *homogeneous* map, meaning that all its keys have the same type as each other (`K`), and all its values have the same type as each other as well (`V`).
- Definition:
	```rust=
	struct Table<phantom K: copy + drop + store, phantom V: store> has key, store {
	    /// the ID of this table
	    id: UID,
	    /// the number of key-value pairs in the table
	    size: u64,
	}
    ```
- 3 instances: 1,367 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Add a key-value pair, mutably borrows the value associated with the key, remove a key-value pair and return the value|Anyone*|

|Create/destroy operations|Who can perform|
|-|-|
|Destroy an empty table, drop a possibly non-empty table|Anyone*|

|Read operations|Who can perform|
|-|-|
|Immutable borrows the value associated with the key, check whether a value associated with the key exists, check whether the table is empty, get the number of key-value pairs|Anyone*|

*\* likely but not sure (couldn't find it)*
- It is not clear where `Table` is made shared.
- Owned objects of type `Table` might exist too.

---

### 3. :package: [`Wormhole`](https://suiexplorer.com/object/0x5306f64e312b581766351c07af79c72fcb1cd25147157fdc2f8ad76de9a3fb6a?network=mainnet)
- Source code: [wormhole](https://github.com/wormhole-foundation/wormhole/tree/main/sui/wormhole)
- Related to the [Pyth](#1.-Pyth) package.
- Shared object types:
    - **`State`** in the `state` module: 1 instance.
- Wormhole is an interoperability protocol powering the seamless transfer of value and information across multiple blockchains.
- Wormhole sends messages cross-chain using a variety of verification methods to attest to the validity of a message. These options are all available to developers in Wormhole’s platform, depending on the chains involved in a given transaction.
- More information on [Wormhole](https://wormhole.com/) and [Wormhole Docs](https://docs.wormhole.com/wormhole/)

#### :m: Module [`state`](https://suiexplorer.com/object/0x5306f64e312b581766351c07af79c72fcb1cd25147157fdc2f8ad76de9a3fb6a?module=state&network=mainnet)
:::info
This module implements the global state variables for Wormhole as a shared object, which performs as a container for all state variables for Wormhole.
:::

##### :red_circle: Type `State`-2
- The `State` object is used to perform anything that requires access to data that defines the Wormhole contract. Examples of which are publishing Wormhole messages (requires depositing a message fee), verifying `VAA` by checking signatures versus an existing Guardian set, and generating new emitters for Wormhole integrators.
- Definition
	```rust=
	struct State has store, key {
	    id: UID,
    	governance_chain: u16,
    	governance_contract: ExternalAddress,
    	guardian_set_index: u32,
    	guardian_sets: Table<u32, GuardianSet>,
    	guardian_set_seconds_to_live: u32,
    	consumed_vaas: ConsumedVAAs,
    	fee_collector: FeeCollector,
    	upgrade_cap: UpgradeCap
    }
	```
- 1 instance: 17,302,581 TXs (in 37,740 transactions - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Deposit fee when sending Wormhole message via `publish_message`, which emits a message as a Sui event. This method uses the input `EmitterCap` as the registered sender of the `WormholeMessage`. It also produces a new sequence number for this emitter. `EmitterCap` which allows one to send Wormhole messages.|Owner of `LatestOnly` and `EmitterCap` capabilities|
|Modify the cost to send a Wormhole message via governance call `set_fee`. This governance message is only relevant for Sui where fee administration is only relevant|Owner of`LatestOnly`|
|Withdraw collected fees via governance call `transfer_fee` to transfer fees to a particular recipient. This governance message is only relevant for Sui where fee administration is only relevant|Owner of `LatestOnly`|
|Store `VAA` hash as a way to claim a VAA. This method prevents a VAA from being replayed. For Wormhole, the only VAAs that it cares about being replayed are its governance actions|Anyone*|
|Update the current Guardian set with a new set of Guardian public keys (`update_guardian_set`). This governance action is applied globally across all networks|Owner of `LatestOnly`|
|Issue an `UpgradeTicket` for the upgrade (`authorize_upgrade`), finalize the upgrade that ran to produce the given `receipt` (`commit_upgrade`). These governance message are only relevant for Sui|Anyone*|
|Roll access from one package to another (`migrate` is called after an upgrade has been commited to add one-off migration logic that would alter Wormhole `State`)|Anyone*|

|Read operations|Who can perform|
|-|-|
|Simple getters: these methods do not require `LatestOnly` for access|Anyone|
|Authorize governance actions, generate a new `EmitterCap` and destroy it|Anyone*|
|`verify_vaa` unpacks a `DecreeTicket` to validate its members to make sure that the parameters match what was encoded in the VAA, `parse_and_verify` parses and verifies the signatures of a VAA|Anyone|
|Obtain a `LatestOnly` capability to interact with `State`, verify that the upgrade contract VAA's encoded package digest used in `migrate` equals the one used to conduct the upgrade|Anyone*|

*\* likely but not sure (couldn't find it)*
- Privileged `State` access is defined by `LatestOnly` (capability reflecting that the current build version is used to invoke state methods), which can only be created within the Wormhole package.
- `State` is a singleton shared object, usually only read in transactions, with dynamic fields, that acts as a container for all state variables for the Wormhole network.
- Because `State` is used to perform anything that requires access to data that defines the Wormhole contract, it seems that most of operations that mutate the shared object can only be performed by the package deployer/maintainer.
- Additionally, some operations that mutate `State` had to be introduced due to package upgradeability only relevant for Sui.

---

### 4. :package: [Kriya DEX](https://suiexplorer.com/object/0xa0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66?network=mainnet)
- Shared object types:
    - **`Pool`** in the `spot_dex` module: 9 instances.
    - **`ProtocolConfigs`** in the `spot_dex` module: 1 instance.
- Kriya is building institutional grade infra for on-chain trading. The current suite includes an in-app bridge, an AMM swap and a fully on-chain orderbook for 20x perp trading.
- Kriya offers an in-app bridge powered by Wormhole Connect that helps you seamlessly bridge funds in and out of the Sui blockchain.
- More info on [Kriya Docs](https://docs.kriya.finance/).

#### :m: Module [`spot_dex`](https://suiexplorer.com/object/0xa0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66?module=spot_dex&network=mainnet)
:::info
Includes key components for a spot Automated Market Maker (AMM) with flexible pricing and Liquidity Provider (LP) rewards. The spot trading infra includes an AMM for native liquidity, a cross protocol-router and integration with DeepBook (Sui's native order book) for pro traders.
:::

##### :red_circle: Type `Pool`
- Definition
	```rust=
    struct Pool<phantom Ty0, phantom Ty1> has key {
		id: UID,
		token_y: Balance<Ty1>,
		token_x: Balance<Ty0>,
		lsp_supply: Supply<LSP<Ty0, Ty1>>,
		lsp_locked: Balance<LSP<Ty0, Ty1>>,
		lp_fee_percent: u64,
		protocol_fee_percent: u64,
		protocol_fee_x: Balance<Ty0>,
		protocol_fee_y: Balance<Ty1>,
		is_stable: bool,
		scaleX: u64,
		scaleY: u64,
		is_swap_enabled: bool,
		is_deposit_enabled: bool,
		is_withdraw_enabled: bool
    }
	```
- 9 instances: 16,075,954 TXs (in 16,075,943 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|`update_pool`, `swap_token_y`, `swap_token_x`, `add_liquidity`, `remove_liquidity`, `update_fees`, `claim_fees`|Anyone*|

*\* likely but not sure (couldn't find it)*

|Read operations|Who can perform|
|-|-|
|`get_reserves`|Anyone|
- There is no any capability implemented, so likely anyone can do any operations with `Pool`.

---

##### :red_circle: Type `ProtocolConfigs`
- Definition
	```rust=
    struct ProtocolConfigs has key {
		id: UID,
		protocol_fee_percent_uc: u64,
		lp_fee_percent_uc: u64,
		protocol_fee_percent_stable: u64,
		lp_fee_percent_stable: u64,
		is_swap_enabled: bool,
		is_deposit_enabled: bool,
		is_withdraw_enabled: bool,
		admin: address,
		whitelisted_addresses: Table<address, bool>
    }
	```
- 1 instance: 17 TXs (in 4 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|`set_stable_fee_config`, `set_uc_fee_config`, `set_pause_config`, `set_whitelisted_address_config`, `remove_whitelisted_address_config`|Admin\*|

*\* likely but not sure (couldn't find it)*

|Read operations|Who can perform|
|-|-|
|`create_pool_`, `update_pool`, `update_fees`|Anyone|
- There is no any capability implemented, but it is likely that only publisher can do mut-ref operations.

---

### 5. :package: [`0xd...1ca`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?network=mainnet)
- Shared object types:
    - **`Storage`** in the `storage` module: 1 instance.
    - **`Pool`** in the `pool` module: 9 instances.
    - **`Incentive`** in the `incentive` module: 1 instance.
    - **`IncentiveBal`** in the `incentive` module: 14 instances.

#### :m: Module [`storage`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=storage&network=mainnet)

##### :red_circle: Type `Storage`
- Definition
	```rust=
    struct Storage has store, key {
		id: UID,
		version: u64,
		paused: bool,
		reserves: Table<u8, ReserveData>,
		reserves_count: u8,
		users: vector<address>,
		user_info: Table<address, UserInfo>
    }
	```
- 1 instance: 4,855,956 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `storage`: `version_migrate`, `init_reserve`, `withdraw_treasury`|Owner of `StorageAdminCap`|
|Module `storage`: `set_pause`, `set_supply_cap`, `set_borrow_cap`, `set_ltv`, `set_treasury_factor`, `set_base_rate`, `set_multiplier`, `set_jump_rate_multiplier`, `set_reserve_factor`, `set_optimal_utilization`, `set_liquidation_ratio`, `set_liquidation_bonus`, `set_liquidation_threshold`|Owner of `OwnerCap`|
|Module `storage`: `get_supply_cap_ceiling`, `get_borrow_cap_ceiling_ratio`, `get_current_rate`, `get_index`, `get_total_supply`, `get_user_balance`, `get_treasury_factor`, `get_borrow_rate_factors`, `get_liquidation_factors`|Anyone*. These seems to be getters.|
|Module `calculator`: `caculate_utilization`, `calculate_borrow_rate`, `calculate_supply_rate`| Anyone*|
|Module `dynamic_calculator`: `dynamic_health_factor`, `dynamic_user_health_collateral_value`, `dynamic_user_health_loan_value`, `dynamic_user_collateral_value`, `dynamic_user_loan_value`, `dynamic_user_collateral_balance`, `dynamic_user_loan_balance`, `dynamic_liquidation_threshold`, `calculate_current_index`, `dynamic_calculate_apy`, `dynamic_calculate_borrow_rate`, `dynamic_calculate_supply_rate`, `dynamic_caculate_utilization`|Anyone*|
|Module `incentive`: `claim_reward`, `earned`|Anyone*|
|Module `lending`: `deposit`, `withdraw`, `borrow`, `repay`, `liquidation_call`|Anyone*|
|Module `logic`: `is_health`, `user_health_factor`, `dynamic_liquidation_threshold`, `user_health_collateral_value`, `user_health_loan_value`, `user_loan_value`, `user_collateral_value`, `user_collateral_balance`, `user_loan_balance`, `is_collateral`, `is_loan`|Anyone*|
|Module `validation`: `validate_deposit`, `validate_withdraw`, `validate_borrow`, `validate_repay`, `validate_liquidate`|Anyone*|

|Read operations|Who can perform|
|-|-|
|`reserve_validation`, `pause`, `get_reserves_count`, `get_user_assets`, `get_oracle_id`, `get_coin_type`, `get_last_update_timestamp`, `get_asset_ltv`, `get_treasury_balance`|Anyone*|

*\* likely but not sure (couldn't find it)*
- Singleton shared object to manage storage of the application.
- Special access is defined by `StorageAdminCap` and `OwnerCap` capabilities.
- Some mut-ref methods seems to be getters.

---

#### :m: Module [`pool`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=pool&network=mainnet)

##### :red_circle: Type `Pool`-2
- Definition
	```rust=
    struct Pool<phantom Ty0> has store, key {
	    id: UID,
	    balance: Balance<Ty0>,
	    treasury_balance: Balance<Ty0>,
	    decimal: u8
    }
	```
- 4 instances: 4,493,331 TXs (in 4,493,326 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `pool`: `withdraw_treasury`|Owner of `PoolAdminCap`|
|Module `storage`: `withdraw_treasury`|Owner of `PoolAdminCap`|
|Module `dynamic_calcultor`: `dynamic_health_factor`, `dynamic_calculate_apy`|Anyone*|
|Module `lending`: `deposit`, `withdraw`, `borrow`, `repay`, `liquidation_call`|Anyone*|

|Read operations|Who can perform|
|-|-|
|Module `pool`: `get_coin_decimal`, `normal_amount`, `unnormal_amount`|Anyone*|

*\* likely but not sure (couldn't find it)*
- Special access is defined by `PoolAdminCap`.

---

#### :m: Module [`incentive`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=incentive&network=mainnet)

##### :red_circle: Type `Incentive`
- Definition
	```rust=
    struct Incentive has store, key {
		id: UID,
		creator: address,
		owners: Table<u256, bool>,
		admins: Table<u256, bool>,
		pools: Table<u8, PoolInfo>,
		assets: vector<u8>
    }
	```
- 1 instance: 4,427,863 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `incentive`: `set_owner`, `set_admin`, `add_pool`, `claim_reward`|Creator, owners, admins*|
|Module `lending`: `deposit`, `withdraw`, `liquidation_call`|Creator, owners, admins*|

|Read operations|Who can perform|
|-|-|
|Module `incentive`: `get_pool_count`, `get_pool_info`, `earned`|Anyone*|

*\* likely but not sure (couldn't find it).*
- Seems anyone can operate on it, no special capability required.

---

##### :red_circle: Type `IncentiveBal`
- Definition
	```rust=
    struct IncentiveBal<phantom Ty0> has store, key {
	    id: UID,
	    asset: u8,
	    current_idx: u64,
	    distributed_amount: u256,
	    balance: Balance<Ty0>
    }
	```
- 14 instances: 368,051 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `incentive`: `claim_reward`|Anyone*|

*\* likely but not sure (couldn't find it)*
- Seems anyone can operate on it, no special capability required.
- Does not have any read-only methods.

---

### 6. :package: [`Cetus 4`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?network=mainnet)
- Source code: [clmmpool](https://github.com/CetusProtocol/cetus-clmm-interface/tree/main/sui/clmmpool)
- Shared object types:
    - **`Pool`** in the `pool` module: 168 instances.
    - **`GlobalConfig`** in the `config` module: 1 instance.
    - **`Partner`** in the `partner` module: 4 instances.
    - **`RewarderGlobalVault`** in the `rewarder` module: 1 instance.
    - **`Pools`** in the `factory` module: 1 instance.
    - **`Partners`** in the `partner` module: 1 instance.
- Cetus is a pioneer DEX and concentrated liquidity protocol built on Sui. It works as a crucial part of the ecosystem infrastructure to satisfy the comprehensive needs of traders, LPs, developers and derivatives products, driven by the increasing population of DeFi.
- Swap, earn, and build on the pioneer Move-based liquidity protocol.
- More info on [Cetus](https://www.cetus.zone/) and [Cetus Docs](https://cetus-1.gitbook.io/cetus-docs/).

#### :m: Module [`pool`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=pool&network=mainnet)

##### :red_circle: Type `Pool`-3
- Concentrated Liquidity Market Maker (CLMM) pool 
- Definition
	```rust=
    struct Pool<phantom Ty0, phantom Ty1> has store, key {
		id: UID,
		coin_a: Balance<Ty0>,
		coin_b: Balance<Ty1>,
		tick_spacing: u32,
		fee_rate: u64,
		liquidity: u128,
		current_sqrt_price: u128,
		current_tick_index: I32,
		fee_growth_global_a: u128,
		fee_growth_global_b: u128,
		fee_protocol_coin_a: u64,
		fee_protocol_coin_b: u64,
		tick_manager: TickManager,
		rewarder_manager: RewarderManager,
		position_manager: PositionManager,
		is_pause: bool,
		index: u64,
		url: String
    }
	```
- 168 instances: 7,493,073 TXs (in 7,493,060 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `pool`: `open_position`, `add_liquidity`, `add_liquidity_fix_coin`, `repay_add_liquidity`, `remove_liquidity`, `close_position`, `collect_fee`, `collect_reward`, `calculate_and_update_rewards`, `calculate_and_update_reward`, `calculate_and_update_points`, `calculate_and_update_fee`, `flash_swap`, `repay_flash_swap`, `flash_swap_with_partner`, `repay_flash_swap_with_partner`, `collect_protocol_fee`, `initialize_rewarder`, `update_emission`, `update_position_url`, `update_fee_rate`, `pause`, `unpause`|Anyone*|

|Read operations|Who can perform|
|-|-|
|Module `pool`: `get_fee_in_tick_range`, `get_rewards_in_tick_range`, `get_points_in_tick_range`, `get_fee_rewards_points_in_tick_range`, `fetch_ticks`, `fetch_positions`, `calculate_swap_result`, `balances`, `tick_spacing`, `fee_rate`, `liquidity`, `current_sqrt_price`, `current_tick_index`, `fees_growth_global`, `protocol_fee`, `tick_manager`, `position_manager`, `rewarder_manager`, `is_pause`, `index`, `url`, `borrow_tick`, `borrow_position_info`, `get_position_fee`, `get_position_points`, `get_position_rewards`, `get_position_reward`, `is_position_exist`|Anyone*|

*\* likely but not sure (couldn't find it)*

---

#### :m: Module [`config`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=config&network=mainnet)

##### :red_circle: Type `GlobalConfig`
- Definition
	```rust=
    struct GlobalConfig has store, key {
	    id: UID,
	    protocol_fee_rate: u64,
	    fee_tiers: VecMap<u32, FeeTier>,
	    acl: ACL,
	    package_version: u64
    }
	```
- 1 instance: 6,093,323 TXs (in 502,222 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `config`: `set_roles`, `add_role`, `remove_role`, `remove_member`, `update_package_version`|Owner of `AdminCap`|
|Module `config`: `update_protocol_fee_rate`, `add_fee_tier`, `delete_fee_tier`, `update_fee_tier`|Anyone*|

|Read operations|Who can perform|
|-|-|
|Module `config`: `get_members`, `get_protocol_fee_rate`, `get_fee_rate`, `check_pool_manager_role`, `check_fee_tier_manager_role`, `check_protocol_fee_claim_role`, `check_partner_manager_role`, `check_rewarder_manager_role`, `protocol_fee_rate`, `fee_tiers`, `acl`, `checked_package_version`|Anyone*|
|Module `factory`: `create_pool`, `create_pool_with_liquidity`|Anyone*|
|Module `partner`: `create_partner`, `update_ref_fee_rate`, `update_time_range`, `claim_ref_fee`|Anyone*|
|Module `partner`: `claim_ref_fee`|Owner of `PartnerCap`|
|Module `pool`: `set_display`, `open_position`, `add_liquidity`, `add_liquidity_fix_coin`, `repay_add_liquidity`, `remove_liquidity`, `close_position`, `collect_fee`, `collect_reward`, `calculate_and_update_rewards`, `calculate_and_update_reward`, `calculate_and_update_points`, `calculate_and_update_fee`, `flash_swap`, `repay_flash_swap`, `flash_swap_with_partner`, `repay_flash_swap_with_partner`, `collect_protocol_fee`, `initialize_rewarder`, `update_emission`, `update_position_url`, `update_fee_rate`, `pause`, `unpause`|Anyone*|
|Module `position`: `set_display`|Anyone*|
|Module `rewarder`: `deposit_reward`|Anyone*|
|Module `rewarder`: `emergent_withdraw`|Owner of `AdminCap`|

*\* likely but not sure (couldn't find it)*

---

#### :m: Module [`partner`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=partner&network=mainnet)

##### :red_circle: Type `Partner`
- Definition
	```rust=
    struct Partner has store, key {
	    id: UID,
	    name: String,
	    ref_fee_rate: u64,
	    start_time: u64,
        end_time: u64,
	    balances: Bag
    }
	```
- 4 instances: 254,373 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `partner`: `update_ref_fee_rate`, `update_time_range`, `receive_ref_fee`|Anyone*|
|Module `partner`: `claim_ref_fee`|Owner of `PartnerCap`|
|Module `pool`: `repay_flash_swap_with_partner`|Anyone*|

|Read operations|Who can perform|
|-|-|
|Module `partner`: `name`, `ref_fee_rate`, `start_time`, `end_time`, `balances`, `current_ref_fee_rate`|Anyone*|
|Module `pool`: `flash_swap_with_partner`|Anyone*|

*\* likely but not sure (couldn't find it)*

---

##### :red_circle: Type `Partners`
- Definition
	```rust=
    struct Partners has key {
	    id: UID,
	    partners: VecMap<String, ID>
    }
	```
- 1 instance: 5 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `partner`: `create_partner`|Anyone*|

*\* likely but not sure (couldn't find it)*

---

#### :m: Module [`rewarder`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=rewarder&network=mainnet)

##### :red_circle: Type `RewarderGlobalVault`
- Pot for rewards.
- Definition
	```rust=
    struct RewarderGlobalVault has store, key {
	    id: UID,
	    balances: Bag
    }
	```
- 1 instance: 216,673 TXs (in 216,581 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `rewarder`: `deposit_reward`|Anyone*|
|Module `rewarder`: `emergent_withdraw`|Owner of `AdminCap`|
|Module `pool`: `collect_reward`|Anyone*|

|Read operations|Who can perform|
|-|-|
|Module `rewarder`: `balances`, `balance_of`|Anyone*|
|Module `pool`: `update_emission`|Anyone*|

*\* likely but not sure (couldn't find it)*

---

#### :m: Module [`factory`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=factory&network=mainnet)

##### :red_circle: Type `Pools`
- Definition
	```rust=
    struct Pools has store, key {
	    id: UID,
        list: LinkedTable<ID, PoolSimpleInfo>,
        index: u64
    }
	```
- 1 instance: 176 TXs (in 174 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `factory`: `create_pool`, `create_pool_with_liquidity`|Anyone*|

|Read operations|Who can perform|
|-|-|
|Module `factory`: `index`, `pool_simple_info`, `fetch_pools`|Anyone*|

*\* likely but not sure (couldn't find it)*

---

### 7. :package: [`DeSuiLabs Coin Flip 2`](https://suiexplorer.com/object/0x745a16ea148a7b3d1f6e68d0f16237f954e99197cd0ffb96e70c994c946d60d1?network=mainnet)
- Source code: [DeSuiCoinFlip-Contract-V2](https://github.com/DeSuiLabs/DeSuiCoinFlip-Contract-V2/tree/main)
- Shared object types:
    - **`Game`** in the `coin_flip` module: 2,702,164 instances.
    - **`HouseData`** in the `coin_flip` module: 1 instance.
- **DeSuiFlip** is a smart contract game for players to double their SUI by guessing heads or tails.
- If a user guesses correctly, they win and the smart contract loses. A loss triggers the smart contract to send the user double their initial bet. However, 1% fee of the original bet amount will be taken.
- If a user guesses incorrectly, the smart contract sends the player's bet into the house wallet.

#### :m: Module [`coin_flip`](https://suiexplorer.com/object/0x745a16ea148a7b3d1f6e68d0f16237f954e99197cd0ffb96e70c994c946d60d1?module=coin_flip&network=mainnet)

##### :red_circle: Type `Game`
- Each guess corresponds to `Game`.
- Definition
    ```rust=
    struct Game has key {
	    id: UID,
        guess_placed_epoch: u64,
	    stake: Balance<SUI>,
        guess: u8,
	    player: address,
        user_randomness: vector<u8>,
	    fee_bp: u16,
        challenged: bool
    }
    ```
- 2,702,164 instances: 4,858,619 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|`play`, `dispute_and_win`|Player*|

*\* likely but not sure (couldn't find it)*

|Read operations|Who can perform|
|-|-|
|`guess_placed_epoch`, `stake`, `guess`, `player`, `player_randomness`, `fee_in_bp`, `challenged`, `fee_amount`|Anyone|

---

##### :red_circle: Type `HouseData`
- `HouseData` is a singleton shared object with administration capability to manage balance, fees, etc. Others (i.e., players) start a new game by adding a dof to the house and settle the game by removing the dof from the house.
- Definition
    ```rust=
    struct HouseData has key {
        id: UID,
	    balance: Balance<SUI>,
        house: address,
   	    public_key: vector<u8>,
        max_stake: u64,
	    min_stake: u64,
	    fees: Balance<SUI>,
	    base_fee_in_bp: u16,
	    reduced_fee_in_bp: u16
    }
    ```
- 1 instance: 3,917,468 TXs (always passed by `&mut`)
- Access control is given by `AdminCap` in the new version
- Only admin can create `HouseData`, during which it is made shared inside its module

|Write operations|Who can perform|
|-|-|
|`top_up`, `withdraw`, `update_max_stake`, `update_min_stake`, `claim_fees`, `start_game`, `start_game_with_capy`, `start_game_with_bullshark`, `start_game_with_dlab`, `play`|Anyone* (In Move byte code, `HouseCap` is destroyed once `HouseData` is created)|
|In the new version: Top-up balance, withdraw from pool, claim treasury, update min & max stake amount, update fee rate|Admin|
|
|Create a new game (this adds the game as a dof to house): start game, start game with partnership, start game with kiosk|Anyone|
|Settle a game or batch of games, challenge - these three remove dof from house|Anyone|


|Read operations|Who can perform|
|-|-|
|`balance`, `house`, `public_key`, `max_stake`, `min_stake`, `fees`, `base_fee_in_bp`, `reduced_fee_in_bp`|Anyone|
|In the new version, getters: public key, fee rate, pool and treasury balances, stake range, whether game exists|Anyone|
|Borrow game|Anyone|

---

### 8. :package: [`ABEx Finance`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?network=mainnet)
- Shared object types:
    - **`Market`** in the `market` module: 1 instance.
    - **`FundingFeeModel`** in the `model` module: 20 instances.
    - **`ReservingFeeModel`** in the `model` module: 3 instances.
    - **`WrappedPositionConfig`** in the `market` module: 19 instance.
    - **`RebaseFeeModel`** in the `model` module: 1 instance.
- ABEx is a revonluationary on-chain derivatives protocol within the Sui ecosystem.
- More about [ABEx](https://abex.fi/)

#### :m: Module [`market`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=market&network=mainnet)

##### :red_circle: Type `Market`
- Definition
	```rust=
    struct Market<phantom Ty0> has key {
		id: UID,
		fun_mask: u256,
		vaults_locked: bool,
		symbols_locked: bool,
		rebate_rate: Rate,
		rebase_fee_model: ID,
		referrals: Table<address, Referral>,
		vaults: Bag,
		symbols: Bag,
		positions: Bag,
		orders: Bag,
		lp_supply: Supply<Ty0>
    }
	```
- 1 instance: 2,202,455 TXs (in 2,202,454 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|`add_new_vault`, `add_new_symbol`, `add_collateral_to_symbol`, `remove_collateral_from_symbol`|Owner of `AdminCap`|
|`add_new_referral`, `open_position`, `liquidate_position`, `execute_open_position_order`, `execute_decrease_position_order`, `deposit`, `withdraw`, `swap`, `create_vaults_valuation`, `create_symbols_valuation`, `valuate_vault`, `valuate_symbol`|Anyone|
|`decrease_position`, `decrease_reserved_from_position`, `pledge_in_position`, `redeem_from_position`, `clear_closed_position`|Owner of `PositionCap`|
|`clear_open_position_order`, `clear_decrease_position_order`|Owner of `OrderCap`|

|Read operations|Who can perform|
|-|-|
|`rebase_fee_model`, `vault`, `symbol`, `position`, `lp_supply_amount`|Anyone|

---

##### :red_circle: Type `WrappedPositionConfig`
- Definition
	```rust=
    struct WrappedPositionConfig<phantom Ty0, phantom Ty1> has key {
	    id: UID,
	    enabled: bool,
	    inner: PositionConfig
    }
	```
- 19 instances: 1,036,392 TXs (always passed by `&`)

|Imm-ref (read-only) operations|Who can perform|
|-|-|
|`open_position`|Anyone|

---

#### :m: Module [`model`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=model&network=mainnet)

##### :red_circle: Type `FundingFeeModel`
- Definition
	```rust=
    struct FundingFeeModel has key {
	    id: UID,
        multiplier: Decimal,
	    max: Rate
    }
	```
- 20 instances: 2,122,883 TXs (always passed by `&`)

|Read operations|Who can perform|
|-|-|
|Module `model`: `compute_funding_fee_rate`|Anyone|
|Module `market`: `open_position`, `liquidate_position`, `execute_open_position_order`, `execute_decrease_position_order`, `valuate_symbol`|Anyone|
|Module `market`: `decrease_position`, `redeem_from_position`|Owner of `PositionCap`|
|Module `pool`: `symbol_delta_funding_rate`|Anyone|

---

##### :red_circle: Type `ReservingFeeModel`
- Definition
	```rust=
    struct ReservingFeeModel has key {
	    id: UID,
	    multiplier: Decimal
    }
	```
- 3 instances: 2,121,559 TXs (always passed by `&`)

|Read operations|Who can perform|
|-|-|
|Module `model`: `compute_reserving_fee_rate`|Anyone|
|Module `market`: `open_position`, `liquidate_position`, `execute_open_position_order`, `execute_decrease_position_order`, `valuate_vault`|Anyone|
|Module `market`: `decrease_position`, `redeem_from_position`, `decrease_reserved_from_position`|Owner of `PositionCap`|
|Module `pool`: `vault_delta_reserving_rate`|Anyone|

---

##### :red_circle: Type `RebaseFeeModel`
- Definition
	```rust=
    struct RebaseFeeModel has key {
	    id: UID,
        base: Rate,
	    multiplier: Decimal
    }
	```
- 1 instance: 833 TXs (in 832 TXs - by `&mut`)

|Read operations|Who can perform|
|-|-|
|Module `model`: `compute_rebase_fee_rate`|Anyone|
|Module `market`: `deposit`, `withdraw`, `swap`|Anyone|
|Module `pool`: `compute_rebase_fee_rate`|Anyone|

---

### 9. :package: [`Turbos Finance 1`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?network=mainnet)
- Source code: [clmm](https://github.com/turbos-finance/turbos-sui-move-interface/tree/main)
- Shared object types:
    - **`Pool`** in the `pool` module: 20 instances.
    - **`Versioned`** in the `pool` module: 1 instances.
    - **`Positions`** in the `position_manager` module: 1 instance.
    - **`PoolRewardVault`** in the `pool` module: 8 instances.
    - **`PoolConfig`** in the `pool_factory` module: 1 instance.
- Turbos Finance is a non-custodial and hyper-efficient DEX, backed by Jump Crypto and Mysten Labs, aspiring to bring a universal notion of digital asset ownership and horizontal scalability to DeFi.
- More info on [Turbos](https://turbos.finance/)

#### :m: Module [`pool`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?module=pool&network=mainnet)
:::info
Defines the Concentrated Liquidity Market Maker (CLMM) pool and related methods.
:::

##### :red_circle: Type `Pool`-4
- Definition
	```rust=
    struct Pool<phantom Ty0, phantom Ty1, phantom Ty2> has store, key {
		id: UID,
		coin_a: Balance<Ty0>,
		coin_b: Balance<Ty1>,
		protocol_fees_a: u64,
		protocol_fees_b: u64,
		sqrt_price: u128,
		tick_current_index: I32,
		tick_spacing: u32,
		max_liquidity_per_tick: u128,
		fee: u32,
		fee_protocol: u32,
		unlocked: bool,
		fee_growth_global_a: u128,
		fee_growth_global_b: u128,
		liquidity: u128,
		tick_map: Table<I32, u256>,
		deploy_time_ms: u64,
		reward_infos: vector<PoolRewardInfo>,
		reward_last_updated_time_ms: u64
    }
	```
- 20 Instances: 3,507,259 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `pool_factory`: `update_pool_fee_protocol`, `collect_protocol_fee`,`toggle_pool_status`|Owner of `PoolFactoryAdminCap`|
|Module `pool_fetcher`: `compute_swap_result`|Anyone|
|Module `position_manager`: `mint`, `increase_liquidity`, `decrease_liquidity`, `collect`, `collect_reward`|Anyone|
|Module `reward_manager`: `init_reward`, `update_reward_manager`|Owner of `RewardManagerAdminCap`|
|Module `reward_manager`: `add_reward`, `remove_reward`, `update_reward_emissions`|Anyone|
|Module `swap_router`: `swap_a_b`, `swap_b_a`, `swap_a_b_b_c`, `swap_a_b_c_b`, `swap_b_a_b_c`, `swap_b_a_c_b`|Anyone|

|Read operations|Who can perform|
|-|-|
|Module `pool`: `get_position`, `get_pool_fee`, `get_pool_sqrt_price`, `get_position_fee_growth_inside_a`, `get_position_base_info`, `get_position_reward_infos`, `get_position_fee_growth_inside_b`, `get_pool_balance`|Anyone|

---

##### :red_circle: Type `Versioned`
- Definition
	```rust=
    struct Versioned has store, key {
	    id: UID,
	    version: u64
    }
	```
- 1 instance: 3,498,499 TXs (in 648 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `pool_factory`: `upgrade`|Owner of `PoolFactoryAdminCap`|

|Read operations|Who can perform|
|-|-|
|Module `pool`: `version`, `check_version`|Anyone|
|Module `pool_factory`: `deploy_pool_and_mint`, `deploy_pool`|Anyone|
|Module `pool_factory`: `set_fee_tier`, `set_fee_protocol`, `update_pool_fee_protocol`, `collect_protocol_fee`, `toggle_pool_status`, `update_nft_name`, `update_nft_description`, `update_nft_img_url`|Owner of `PoolFactoryAdminCap`|
|Module `pool_fetcher`: `compute_swap_result`|Anyone|
|Module `position_manager`: `mint`, `burn`, `increase_liquidity`, `decrease_liquidity`, `collect`, `collect_reward`|Anyone|
|Module `reward_manager`: `init_reward`, `update_reward_manager`|Owner of `RewardManagerAdminCap`|
|Module `reward_manager`: `add_reward`, `remove_reward`, `update_reward_emissions`|Anyone|
|Module `swap_router`: `swap_a_b`, `swap_b_a`, `swap_a_b_b_c`, `swap_a_b_c_b`, `swap_b_a_b_c`, `swap_b_a_c_b`|Anyone|

---

##### :red_circle: Type `PoolRewardVault`
- Definition
	```rust=
    struct PoolRewardVault<phantom Ty0> has store, key {
	    id: UID,
	    coin: Balance<Ty0>
    }
	```
- 8 instances: 191,318 TXs (always passed by a `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `position_manager`: `collect_reward`|Anyone|
|Module `reward_manager`: `add_reward`, `remove_reward`|Anyone|

---

#### :m: Module [`position_manager`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?module=position_manager&network=mainnet)

##### :red_circle: Type `Positions`
- Definition
	```rust=
	struct Positions has store, key {
		id: UID,
		nft_minted: u64,
		user_position: Table<address, ID>,
		nft_name: String,
		nft_description: String,
		nft_img_url: String
    }
	```
- 1 instance: 215,522 TXs (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `pool_factory`: `deploy_pool_and_mint`|Anyone|
|Module `pool_factory`: `update_nft_name`, `update_nft_description`, `update_nft_img_url`|Owner of `PoolFactoryAdminCap`|
|Module `position_manager`: `mint`, `burn`, `increase_liquidity`, `decrease_liquidity`, `collect`, `collect_reward`|Anyone|

---

#### :m: Module [`pool_factory`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?module=pool_factory&network=mainnet)

##### :red_circle: Type `PoolConfig`
- Definition
	```rust=
	struct PoolConfig has store, key {
	    id: UID,
	    fee_map: VecMap<String, ID>,
	    fee_protocol: u32,
	    pools: Table<ID, PoolSimpleInfo>
    }
	```
- 1 instance: 25 transactions (always passed by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `pool_factory`: `deploy_pool_and_mint`, `deploy_pool`|Anyone|
|Module `pool_factory`: `set_fee_tier`, `set_fee_protocol`|Owner of `PoolFactoryAdminCap`|

---

### 10. :package: [`Scallop`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?network=mainnet)
- Source code: [sui-lending-protocol](https://github.com/scallop-io/sui-lending-protocol)
- Shared object types:
    - **`Version`** in the `version` module: 1 instance
    - **`Market`** in the `market` module: 1 instance
    - **`Obligation`** in the `Obligation` module: 55,901 instances
- Scallop is the Next Generation Interest Rate Machine for the Sui ecosystem
- Scallop developers are dedicated to building a dynamic money market that offers high-interest lending, low-fee borrowing, AMM, and digital asset self-administration tool on a unified platform and offering an SDK for professional traders.
- More info on [Scallop](https://www.scallop.io/)

#### :m: Module [`version`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?module=version&network=mainnet)

##### :red_circle: Type `Version`
- Definition
	```rust=
    struct Version has store, key {
	    id: UID,
	    value: u64
    }
	```
- 1 instance: 2,832,904 TXs (in 11,593 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `version`: `upgrade`|Owner of `VersionCap`|

|Read operations|Who can perform|
|-|-|
|Module `version`: `value`, `is_current_version`, `assert_current_version`|Anyone|
|Module `accrue_interest`: `accrue_interest_for_market`, `accrue_interest_for_market_and_obligation`|Anyone|
|Module `borrow`: `borrow_entry`, `borrow`|Anyone with `ObligationKey`|
|Module `deposit_collateral`: `deposit_collateral`|Anyone|
|Module `flash_loan`: `borrow_flash_loan`, `repay_flash_loan`|Anyone|
|Module `liquidate`: `liquidate_entry`, `liquidate`|Anyone|
|Module `mint`: `mint_entry`, `mint`|Anyone|
|Module `open_obligation`: `open_obligation_entry`, `open_obligation`, `return_obligation`|Anyone|
|Module `redeem`: `redeem_entry`, `redeem`|Anyone|
|Module `repay`: `repay`|Anyone|
|Module `withdraw_collateral`: `withdraw_collateral_entry`, `withdraw_collateral`|Anyone with `ObligationKey`|

---

#### :m: Module [`market`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?module=market&network=mainnet)

##### :red_circle: Type `Market`-2
- Definition
	```rust=
    struct Market has store, key {
		id: UID,
		borrow_dynamics: WitTable<BorrowDynamics, TypeName, BorrowDynamic>,
		collateral_stats: WitTable<CollateralStats, TypeName, CollateralStat>,
		interest_models: AcTable<InterestModels, TypeName, InterestModel>,
		risk_models: AcTable<RiskModels, TypeName, RiskModel>,
		limiters: WitTable<Limiters, TypeName, Limiter>,
		reward_factors: WitTable<RewardFactors, TypeName, RewardFactor>,
		asset_active_states: AssetActiveStates,
		vault: Reserve
    }
	```
- 1 instance: 2,813,745 TXs (in 2,813,719 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `market`: `uid_mut_delegated`|Anyone with `Witness<Market>`|
|Module `accrue_interest`: `accrue_interest_for_market`, `accrue_interest_for_market_and_obligation`|Anyone|
|Module `app`: `ext`, `add_whitelist_address`, `remove_whitelist_address`, `add_interest_model`, `update_interest_model`, `add_risk_model`, `update_risk_model`, `add_limiter`, `apply_limiter_limit_change`, `apply_limiter_params_change`, `set_incentive_reward_factor`, `set_flash_loan_fee`, `set_base_asset_active_state`, `set_collateral_active_state`, `take_revenue`|Owner of `AdminCap`|
|Module `borrow`: `borrow_entry`, `borrow`|Anyone with `ObligationKey`|
|Module `deposit_collateral`: `deposit_collateral`|Anyone|
|Module `flash_loan`: `borrow_flash_loan`, `repay_flash_loan`|Anyone|
|Module `liquidate`: `liquidate_entry`, `liquidate`|Anyone|
|Module `mint`: `mint_entry`, `mint`|Anyone|
|Module `redeem`: `redeem_entry`, `redeem`|Anyone|
|Module `repay`: `repay`|Anyone|
|Module `withdraw_collateral`: `withdraw_collateral_entry`, `withdraw_collateral`|Anyone with `ObligationKey`|

|Read operations|Who can perform|
|-|-|
|Module `market`: `uid`, `borrow_dynamics`, `interest_models`, `vault`, `risk_models`, `reward_factors`, `collateral_stats`, `borrow_index`, `interest_model`, `risk_model`, `reward_factor`, `has_risk_model`, `has_limiter`, `is_base_asset_active`, `is_collateral_active`|Anyone|
|Module `borrow_withdraw_evaluator`: `available_borrow_amount_in_usd`, `max_borrow_amount`, `max_withdraw_amount`|Anyone|
|Module `collateral_value`: `collaterals_value_usd_for_borrow`, `collaterals_value_usd_for_liquidation`|Anyone|
|Module `debt_value`: `debts_value_usd_with_weight`|Anyone|
|Module `liquidation_evaluator`: `liquidation_amounts`, `max_liquidation_amounts`|Anyone|

---

#### :m: Module [`obligation`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?module=obligation&network=mainnet)

##### :red_circle: Type `Obligation`
- Definition
	```rust=
    struct Obligation has store, key {
		id: UID,
		balances: BalanceBag,
		debts: WitTable<ObligationDebts, TypeName, Debt>,
		collaterals: WitTable<ObligationCollaterals, TypeName, Collateral>,
		rewards_point: u64,
		lock_key: Option<TypeName>,
		borrow_locked: bool,
		repay_locked: bool,
		deposit_collateral_locked: bool,
		withdraw_collateral_locked: bool,
		liquidate_locked: bool
    }
	```
- 55,901 instances: 251,184 TXs (always by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `obligation`: `obligation_uid_mut_delegated`|Anyone|
|Module `obligation`: `lock`, `unlock`, `redeem_rewards_point`|Owner of `ObligationKey`|
|Module `accrue_interest`: `accrue_interest_for_market_and_obligation`|Anyone|
|Module `borrow`: `borrow_entry`, `borrow`|Owner of `ObligationKey`|
|Module `deposit_collateral`: `deposit_collateral`|Anyone|
|Module `liquidate`: `liquidate_entry`, `liquidate`|Anyone|
|Module `repay`: `repay`|Anyone|
|Module `withdraw_collateral`: `withdraw_collateral_entry`, `withdraw_collateral`|Owner of `ObligationKey`|

|Read operations|Who can perform|
|-|-|
|Module `obligation`: `obligation_uid`, `debt`, `collateral`, `debt_types`, `collateral_types`, `balance_bag`, `debts`, `collaterals`, `rewards_point`, `borrow_locked`, `repay_locked`, `withdraw_collateral_locked`, `deposit_collateral_locked`, `liquidate_locked`, `lock_key`|Anyone|
|Module `obligation`: `assert_key_match`, `is_key_match`|Owner of `ObligationKey`|
|Module `borrow_withdraw_evaluator`: `available_borrow_amount_in_usd`, `max_borrow_amount`, `max_withdraw_amount`|Anyone|
|Module `collateral_value`: `collaterals_value_usd_for_borrow`, `collaterals_value_usd_for_liquidation`|Anyone|
|Module `debt_value`: `debts_value_usd`, `debts_value_usd_with_weight`|Anyone|
|Module `liquidation_evaluator`: `liquidation_amounts`, `max_liquidation_amounts`|Anyone|

---

### 11. :package: [`DeepBook`](https://suiexplorer.com/object/0x000000000000000000000000000000000000000000000000000000000000dee9?network=mainnet)
- Source code: [deepbook](https://github.com/MystenLabs/sui/tree/main/crates/sui-framework/packages/deepbook)
- Shared object types:
    - **`Pool`** in the `clob_v2` module: 5 instances.
- DeepBook is a decentralized central limit order book (CLOB) built for the Sui ecosystem.
- DeepBook provides a one-stop shop for trading digital assets, with a technical design built for Sui’s architecture.
- Designed as permissionless and released as open source, DeepBook will accelerate the development of financial and other apps on Sui. It will give builders an efficient and shared ready-built financial layer for trading fungible assets. 

#### :m: Module [`clob_v2`](https://suiexplorer.com/object/0x000000000000000000000000000000000000000000000000000000000000dee9?module=clob_v2&network=mainnet)

##### :red_circle: Type `Pool`-5
- Definition
	```rust=
    struct Pool<phantom Ty0, phantom Ty1> has store, key {
		id: UID,
		bids: CritbitTree<TickLevel>,
		asks: CritbitTree<TickLevel>,
		next_bid_order_id: u64,
		next_ask_order_id: u64,
		usr_open_orders: Table<address, LinkedTable<u64, u64>>,
		taker_fee_rate: u64,
		maker_rebate_rate: u64,
		tick_size: u64,
		lot_size: u64,
		base_custodian: Custodian<Ty0>,
		quote_custodian: Custodian<Ty1>,
		creation_fee: Balance<SUI>,
		base_asset_trading_fees: Balance<Ty0>,
		quote_asset_trading_fees: Balance<Ty1>
    }
	```
- 5 instances: 5,537,010 TXs (in 5,537,006 TXs - by `&mut`)

|Write operations|Who can perform|
|-|-|
|Module `clob_v2`: `withdraw_fees`|Owner of `PoolOwnerCap`|
|Module `clob_v2`: `deposit_base`, `deposit_quote`, `withdraw_base`, `withdraw_quote`, `swap_exact_base_for_quote`, `swap_exact_quote_for_base`, `place_market_order`, `place_limit_order`, `cancel_order`, `cancel_all_orders`, `batch_cancel_order`, `clean_up_expired_orders`|Anyone with `AccountCap`|

- `PoolOwnerCap` grants permission to access an entry in `Pool.quote_asset_trading_fees`
- `AccountCap` grants permission to access an entry in [`Custodian.account_balances`](https://github.com/MystenLabs/sui/blob/aa333098ce8257c42cd7c90b7a32608e316d4ab8/crates/sui-framework/packages/deepbook/sources/custodian_v2.move#L37)

|Read operations|Who can perform|
|-|-|
|Module `clob_v2`: `usr_open_orders_exist`, `usr_open_orders_for_address`, `usr_open_orders`, `get_market_price`, `get_level2_book_status_bid_side`, `get_level2_book_status_ask_side`, `asks`, `bids`, `tick_size`, `maker_rebate_rate`, `taker_fee_rate`, `pool_size`, `quote_asset_trading_fees_value`|Anyone|
|Module `clob_v2`: `list_open_orders`, `account_balance`, `get_order_status`|Anyone with `AccountCap`|
|Module `order_query`: `iter_asks`, `iter_bids`|Anyone|

---

## Shared object classification

Sui classifies objects into owned objects, immutable objects, and shared objects. Having a list of (the most used) Sui shared objects, we would like to investigate how one could make a more "fine-grained" classification of shared objects to minimize their involvement.

We have identified the most important properties that can be used to make a detailed classification of Sui shared objects. These properties are based on whether a shared object type:
- (i) has `store` ability or not;
- (ii) can be a singleton or multiton;
- (iii) is always passed by a mutable or immutable reference or both;
- (iv) has owned or immutable objects too;
- (v) has an owner-like logic; 
- (vi) can be written by anyone or someone with privilege.

### 1. Has the `store` ability or not
Sui shared objects can be classified based on whether they have the **`store` ability** (which is equivalent to (i) having public transfer, and (ii) being resources) or not.

The definition of a resource is given in the [Move book](https://move-book.com/resources/what-is-resource.html):
> Resource is a concept described in Move Whitepaper. Originally it was implemented as its own type but later, with addition of abilities, replaced with two abilities: Key and Store. Resource is meant to be a perfect type for storing digital assets, to achieve that it must to be non-copyable and non-droppable. At the same time it must be storable and transferable between accounts.
>
> **Definition**
> Resource is a struct that has only `key` and `store` abilities.

Additionally, having `store` ability defines how the object can be made shared and whether that object can be publicly transferred or not:
1. If the object does not have the `store` ability, it can only be made shared inside its module by using the [`share_object`](https://suiexplorer.com/object/0x2?module=transfer&network=mainnet) method. Shared objects without the `store` ability cannot be publicly transferred.
2. If the object has the `store` ability, it is can be made shared both inside its module by using [`share_object`](https://suiexplorer.com/object/0x2?module=transfer&network=mainnet) and outside of its module by using [`public_share_object`](https://suiexplorer.com/object/0x2?module=transfer&network=mainnet). Shared objects with the `store` ability can be publicly transferred.
    - An example of an object with the `store` ability that is made shared inside its module via `share_object` is [`Kiosk`](https://github.com/MystenLabs/sui/blob/main/crates/sui-framework/packages/sui-framework/sources/kiosk/kiosk.move).

### 2. Singleton or Multiton
A Sui shared object can also be classified based on whether it is a **singleton** (only one instance of that shared object type was ever created) or a **multiton** (more than one instance of that shared object type have been created).

:::danger
**NOTE:** If after analysis of the Sui network until some epoch there are only one instance of a shared object type, this does not mean another instance of that type will not appear in the future epochs. There must be a proper way to check whether a shared object is a singleton or not, like it could be checked for `clock.Clock`.
:::

### 3. Passing by a mutable reference
Sui shared objects can also be distinguished whether they are always passed by an immutable reference or not. If a shared object has never been passed by a mutable reference, we might want to know what entities can mutate that object.

:::danger
**NOTE:** If after analysis of the Sui network until some epoch there is a shared object that has never been passed by a mutable reference, this does not mean that object will not be passed by a mutable reference in the future epochs. There must be a proper way to check whether a shared object can only be passed by an immutable reference or not, like it could be checked for `clock.Clock`.
:::

### 4. Owned or immutable objects of the same type exist
It might be the case that owned or shared objects of the same type as that of a shared object exist, especially in the case of multiton shared objects.

### 5. Owner-like logic
Some shared objects might have an owner-like logic/capability implemented, which gives the bearer of that capability the full/privileged access to 
a shared object.

### 6. Who can write it
Since object contention arise due to simultaneous writes, it is important to investigate who can write the object. It might be the case that anyone is eligible to modify a shared object, or there is a set of access control rules implemented to give write privilege only to some entities.

:::warning
Interesting [Sui Improvement Proposals 2.0](https://fidika.medium.com/sui-improvement-proposals-2-0-110973ef1fbb) that proposes a better terminology for Sui owned vs shared objects and some other improvements.
:::

### Summarizing table
|Shared object type|Module|TX num (mut)|(i) `store`|(ii) Multiton|(iii) `&mut`|(iv) Also `ImmOrOwned`|(v) Owner-like|(vi) Who can write|
|-|-|-|-|-|-|-|-|-|
|[`PriceInfoObject`](#-Type-PriceInfoObject)|[`price_info`](https://suiexplorer.com/object/0x00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302?module=price_info&network=mainnet)|77,826,890 (75,629,251)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Anyone|
|[`State`](#-Type-State-1)|[`state`](https://suiexplorer.com/object/0x00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302?module=state&network=mainnet)|16,696,329 (10,931)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Bearer of `DecreeReceipt` and/or `LatestOnly`|
||||||||||
|[`Clock`](#-Type-Clock)|[`clock`](https://suiexplorer.com/object/0x2?module=clock&network=mainnet)|44,596,744 (0)|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Only Sui validators|
|[`Kiosk`](#-Type-Kiosk)|[`kiosk`](https://suiexplorer.com/object/0x2?module=kiosk&network=mainnet)|4,581,703 (1,060,302)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|Many operations can only be done by owner of `KioskOwnerCap`. Anyone can purchase an item from kiosk|
|[`TransferPolicy`](#-Type-TransferPolicy)|[`transfer_policy`](https://suiexplorer.com/object/0x2?module=transfer_policy&network=mainnet) |691,784 (448,988)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign: |Many operations can only be done by owner of `TransferPolicyCap`. Anyone can only add some SUI to the balance|
|[`TreasuryCap`](#-Type-TreasuryCap)|[`coin`](https://suiexplorer.com/object/0x2?module=coin&network=mainnet)|4,671 (4,671)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_plus_sign:|:heavy_plus_sign:|Only owner\*\*|
|[`Table`](#-Type-Table)|[`table`](https://suiexplorer.com/object/0x2?module=table&network=mainnet)|1,367 (1,367)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:\*\*|:heavy_minus_sign:\*\*|Anyone|
|[`CoinMetadata`](#-Type-CoinMetadata)|[`coin`](https://suiexplorer.com/object/0x2?module=coin&network=mainnet)|397 (190)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|Owner of `TreasuryCap<T>`|
||||||||||
|[`State`](#-Type-State-2)|[`state`](https://suiexplorer.com/object/0x5306f64e312b581766351c07af79c72fcb1cd25147157fdc2f8ad76de9a3fb6a?module=state&network=mainnet)|17,302,581 (37,740)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Some operations can be done by anyone, but many require `LatestOnly`, `EmitterCap`, `DeployerCap`|
||||||||||
|[`Pool`](#-Type-Pool)|[`spot_dex`](https://suiexplorer.com/object/0xa0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66?module=spot_dex&network=mainnet)|16,075,954 (16,075,943)|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Anyone|
|[`ProtocolConfigs`](#-Type-ProtocolConfigs)|[`spot_dex`](https://suiexplorer.com/object/0xa0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66?module=spot_dex&network=mainnet)|17 (4)|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Admin|
||||||||||
|[`Storage`](#-Type-Storage)|[`storage`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=storage&network=mainnet)|4,855,956 (4,855,956)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:| Some operations require `StorageAdminCap` and `OwnerCap`, but many others can be done by anyone|
|[`Pool`](#-Type-Pool-2)|[`pool`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=pool&network=mainnet)|4,493,331 (4,493,326)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Some operations require `PoolAdminCap`, others can be done by anyone|
|[`Incentive`](#-Type-Incentive)|[`incentive`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=incentive&network=mainnet)|4,427,863 (4,427,863)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Creator, owners, admins\*\*|
|[`IncentiveBal`](#-Type-IncentiveBal)|[`incentive`](https://suiexplorer.com/object/0xd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca?module=incentive&network=mainnet)|368,051 (368,051)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_minus_sign:|Anyone**. Does not have any read-only operations|
||||||||||
|[`Pool`](#-Type-Pool-3)|[`pool`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=pool&network=mainnet)|7,493,073 (7,493,073)| :heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_minus_sign:|Anyone|
|[`GlobalConfig`](#-Type-GlobalConfig)|[`config`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=config&network=mainnet)|6,093,323 (502,222)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Some operations require `AdminCap`, some can be done by anyone|
|[`Partner`](#-Type-Partner)|[`partner`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=partner&network=mainnet)|254,373 (254,373)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Some operations require `PartnerCap`, but many can be done by anyone|
|[`Partners`](#-Type-Partners)|[`partner`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=partner&network=mainnet)|5 (5)|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_minus_sign:|Anyone. Does not have any read-only operations|
|[`RewarderGlobalVault`](#-Type-RewarderGlobalVault)|[`rewarder`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=rewarder&network=mainnet)|216,673 (216,581)| :heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Some operations require `AdminCap`, but many can be done by anyone|
|[`Pools`](#-Type-Pools)|[`factory`](https://suiexplorer.com/object/0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb?module=factory&network=mainnet)|176 (174)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Anyone|
||||||||||
|[`Game`](#-Type-Game)|[`coin_flip`](https://suiexplorer.com/object/0x745a16ea148a7b3d1f6e68d0f16237f954e99197cd0ffb96e70c994c946d60d1?module=coin_flip&network=mainnet)|4,858,619 (4,858,619)|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:\*\*|:heavy_plus_sign:|Player**|
|[`HouseData`](#-Type-HouseData)|[`coin_flip`](https://suiexplorer.com/object/0x745a16ea148a7b3d1f6e68d0f16237f954e99197cd0ffb96e70c994c946d60d1?module=coin_flip&network=mainnet)|3,917,468 (3,917,468)|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Many operations require `AdminCap`, players can some `Game`-related operations|
||||||||||
|[`Market`](#-Type-Market)|[`market`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=market&network=mainnet)|2,202,455 (2,202,454)|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Some operations require `AdminCap`, `PositionCap`, `OrderCap`, but many can be done by anyone|
|[`WrappedPositionConfig`](#-Type-WrappedPositionConfig)|[`market`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=market&network=mainnet)|1,036,392 (0)|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Nobody. Does not have any write operations|
|[`FundingFeeModel`](#-Type-FundingFeeModel)|[`model`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=model&network=mainnet)|2,122,883 (0)|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Nobody. Does not have any write operations|
|[`ReservingFeeModel`](#-Type-ReservingFeeModel)|[`model`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=model&network=mainnet)|2,121,559 (0)|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Nobody. Does not have any write operations|
|[`RebaseFeeModel`](#-Type-RebaseFeeModel)|[`model`](https://suiexplorer.com/object/0xceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f?module=model&network=mainnet)|833 (1)|:heavy_minus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_minus_sign:|Nobody. Does not have any write operations|
||||||||||
|[`Pool`](#-Type-Pool-4)|[`pool`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?module=pool&network=mainnet)|3,507,259 (3,507,259)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Many operations can be done by anyone, but some require `PoolFactoryAdminCap` and `RewardManagerAdminCap`|
|[`Versioned`](#-Type-Versioned)|[`pool`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?module=pool&network=mainnet)|3,498,499 (648)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Only owner of `PoolFactoryAdminCap`|
|[`PoolRewardVault`](#-Type-PoolRewardVault)|[`pool`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?module=pool&network=mainnet)|191,318 (191,318)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_minus_sign:|Anyone. Does not have any read-only operations|
|[`Positions`](#-Type-Positions)|[`position_manager`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?position_manager=pool&network=mainnet)|215,522 (215,522)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Some operations can be done by anyone, some require `PoolFactoryAdminCap`. Does not have any read-only operations|
|[`PoolConfig`](#-Type-PoolConfig)|[`pool_factory`](https://suiexplorer.com/object/0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1?pool_factory=pool&network=mainnet)|25 (25)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Some operations can be done by anyone, some require `PoolFactoryAdminCap`. Does not have any read-only operations|
||||||||||
|[`Version`](#-Type-Version)|[`version`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?module=version&network=mainnet)|2,832,904 (11,593)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Only owner of `VersionCap`|
|[`Market`](#-Type-Market-2)|[`market`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?module=market&network=mainnet)|2,813,745 (2,813,719)|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Many operations require `AdminCap`, some can be done by anyone|
|[`Obligation`](#-Type-Obligation)|[`obligation`](https://suiexplorer.com/object/0xefe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf?module=obligation&network=mainnet)|251,184 (251,184)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:\*|:heavy_minus_sign:|:heavy_plus_sign:|Some operations can be done by anyone, some require `ObligationKey`|
||||||||||
|[`Pool`](#-Type-Pool-5)|[`clob_v2`](https://suiexplorer.com/object/0x000000000000000000000000000000000000000000000000000000000000dee9?module=clob_v2&network=mainnet)|5,537,010 (5,537,006)|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_plus_sign:|:heavy_minus_sign:|:heavy_plus_sign:|Some operation require `PoolOwnerCap`, but many can be done by anyone with `AccountCap` for a given order|

*\* always passed by a mutable reference.*
*\*\* likely but not sure (couldn't find it).*
