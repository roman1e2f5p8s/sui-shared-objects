# Sui Shared Objects

[sui-shared-objects](https://github.com/roman1e2f5p8s/sui-shared-objects) 
is a Rust-based project that provides convenient tools to estimate and 
visualize how often transactions operate on shared objects on the [Sui 
network](https://sui.io/), using the [Sui Rust 
SDK](https://docs.sui.io/references/rust-sdk). We also list popular dApps 
that utilize shared objects on Sui.

## Table of Contents

- [Background](#background)
    - [Motivation](#motivation)
- [Getting Started](#getting-started)
    - [Install](#install)
- [Usage](#usage)
- [Examples](#examples)
- [Results](#results)
- [Contribute](#contribute)
- [License](#license)

## Background

[Sui](https://docs.sui.io/paper/sui.pdf) is a layer-1 smart contract platform 
that utilizes an [object-centric data 
model](https://docs.sui.io/concepts/object-model): the basic unit 
of storage in Sui is an **object**. The Sui ledger, therefore, stores a 
collection of programmable objects, each with a globally unique ID.

From the ownership point of view, there are two types of objects in Sui:
- [**Owned 
objects**](https://docs.sui.io/concepts/object-ownership/address-owned): owned 
by an address and can only be used by transactions signed by that owner 
address at a time. 
- [**Shared objects**](https://docs.sui.io/concepts/object-ownership/shared): 
do not have a specific owner; anyone can read or write these objects.

Mutable owned objects are **single-writers**, and thus, transactions involving 
only owned objects may bypass consensus on sequencing in Sui. Mutable shared 
objects (**multi-writers**), however, require consensus to sequence (order) 
reads and writes. See [Sui Lutris](https://arxiv.org/abs/2310.18042) for 
detail.

### Motivation

Owned objects are the most common case in Sui. According to the (*old*) [Sui 
documentation](https://github.com/MystenLabs/sui/blob/21dad3ec1f2caf03ac4310e8e033fd6987c392bf/doc/src/learn/single-writer-apps.md), 
many transactions (e.g., asset transfers, NFT minting, smart contract 
publishing) can be realized involving only owned objects.

> [!WARNING]
> This list of single-writer applications was once provided on the Sui 
> documentation at https://docs.sui.io/learn/single-writer-apps but removed 
> at some point later. If it cannot be found in the [`21dad3e` 
> commit](https://github.com/MystenLabs/sui/blob/21dad3ec1f2caf03ac4310e8e033fd6987c392bf/doc/src/learn/single-writer-apps.md) 
> on the Sui repo, we archived it [here](./single-writer-apps.md).

However, 
[Sui documentation](https://docs.sui.io/learn/how-sui-works#transactions-on-shared-objects) 
also claims that many use cases require shared objects that can be manipulated by two or 
more addresses at once (e.g., an auction with open bidding). Therefore, it is not clear how 
often Sui transactions actually touch shared objects, i.e., what the value of the density is. 

Estimating the density in the Sui network would give one an insight into how frequently the use
cases that require shared objects appear on the Sui smart contract platform. The interest in 
knowing the density stems from the fact that transactions with shared object inputs 
require sequencing via the consensus protocol. Therefore, understanding how many apps require
operating with shared objects and what those use cases are is one of the first key steps in 
designing an efficient smart contract architecture.

## Shared Object Analysis Rationale

To analyse a given Sui shared object, different object data options might be used, as specified in
[SuiObjectDataOptions](https://github.com/MystenLabs/sui/blob/2456e2888c15fd843be3370d395f18cafb753563/crates/sui-json-rpc-types/src/sui_object.rs#L326).
With the `show_content` option, a query returns the module name, the object name, the 
`has_public_transfer` field (which indicates whether the object is shared outside 
of its module), among others.

When an object is turned into a mutable shared object, 
there are two possibilities for the scope of the shared object:
(1) it can be shared only within its module, or 
(2) it can be publicly shared outside of its module. The object must have the 
[`store`](https://github.com/MystenLabs/sui/blob/284bf584b46bc3704d0c48cf478923987749a665/sui-execution/latest/sui-adapter/src/programmable_transactions/context.rs#L119)
ability in order to be shared outside of its module. See `shared_object` and
`public_share_object` functions in the 
[`transfer` module](https://suiexplorer.com/object/0x0000000000000000000000000000000000000000000000000000000000000002?module=transfer&network=mainnet)
for more detail.

The module name and the object name can be used to determine the type of the shared object
and which applications use Sui shared objects.
The `has_public_transfer` field can be used to determine which shared objects are shared
outside of their modules and whether they are *resources* or not. 
Recall that a [resource in Move](https://move-book.com/resources/what-is-resource.html) 
is a struct that has only `key` and `store` abilities. Therefore, a shared object with 
`has_public_transfer: true` is a shared resource (publicly shared outside of its module),
while shared objects with `has_public_transfer: false` are not resources 
(they are shared only inside of its module and they might be Sui system/"protocol" shared objects).

## Getting Started

Before you start, please refer to [Sui Rust SDK documentation](https://docs.sui.io/build/rust-sdk)
and 
[Sui Rust SDK source code](https://github.com/MystenLabs/sui/blob/main/crates/sui-sdk/src/apis.rs)
if you need more information about Sui Rust SDK and available API methods it contains.

### Install

This project assumes `Rust` and `Cargo` are installed, and that there is an available 
internet connection. Please refer to the 
[Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) 
for the installation instructions.

- Clone the project:
```bash
git clone https://github.com/roman1e2f5p8s/sui-shared-object-density.git
cd sui-shared-object-density
```

- Build the project:
```bash
cargo build --release
```

## Usage
Building the project shall create three executable files:
- `query_txs`;
- `density`;
- `query_obj`.

On UNIX-like systems they can be executed using `./target/release/<NAME>`, and
on Windows - `.\target\release\<NAME>.exe`, where `<NAME>` is one of the three executables 
listed above.

A more detailed description of how to use these executables and what they do are given in the next sub-sections.

### `query_txs`
Use `query_txs` to query all the transactions for a given epoch, and pre-process them.

For example,
```bash
./target/release/query_txs --epoch=0
```
will query all transactions from epoch `0` and pre-process them according to the data
structure specified in TODO.

By default, the requested and processed data will be saved in `data/workspace1/`, one file 
per epoch. You can create another workspace using the `--workspace` command line
argument for `query_txs`.

See `./target/release/query_txs --help` for more information and all arguments.

See TODO for the description and explanation of which data about Sui transactions `query_txs`
collects.

### `density`
TODO

### `query_obj`
TODO

## Results
We plot the following characteristics:
- **Total number of transactions** is the total number of Sui transactions per epoch.
- **Number of transactions touching shared objects** is the number (per epoch) of Sui transactions 
that have at least one shared object in their inputs.
- **Density** is the percentage of Sui transactions that touch shared objects, i.e.,
the ratio (multiplied by 100%) of the number of transactions touching shared objects 
(per epoch) to the total number of transactions on Sui (per epoch).
- **Number of shared objects** is the number of shared objects with unique IDs within an epoch.
- **Average contention degree** is the ratio (averaged over intervals within an epoch) of 
the number of transactions touching shared objects to the number of unique shared objects
touched by those transactions within an interval. In other words, this metrics 
tells us how many transactions touch the same shared object on average.
- **Object touchability** is the ratio (averaged over intervals within an epoch) of
the number of shared objects touched by more than one transaction to
the number of unique shared objects. 
