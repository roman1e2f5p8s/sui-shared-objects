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
- [Metrics](#metrics)
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

On the other hand, the (*old*) [Sui documentation](
https://github.com/MystenLabs/sui/blob/21dad3ec1f2caf03ac4310e8e033fd6987c392bf/doc/src/learn/how-sui-works.md#transactions-on-shared-objects) 
also claims that many use cases require shared objects that can be manipulated 
by two or more addresses at once (e.g., an auction with open bidding, a 
central limit order book that accepts arbitrary trades). Thus, it is 
reasonable to investigate how often Sui transactions actually operate on 
shared objects. 

This analysis of the Sui network may give insights into how 
frequently the use cases that require shared objects appear on the Sui smart 
contract platform. The interest in this analysis stems from the fact that 
transactions with shared object inputs require sequencing via the consensus 
protocol. Therefore, understanding how often apps involve shared objects and 
what those use cases are is one of the first key steps in improving the 
efficiency of object-based smart contract architectures. This analysis is 
relevant for smart contract platform designers and smart contract developers.

## Getting Started

Before you start, please refer to the [Sui Rust SDK 
documentation](https://docs.sui.io/references/rust-sdk) and/or the [Sui Rust 
SDK source code](https://github.com/MystenLabs/sui/blob/main/crates/sui-sdk/src/apis.rs)
if you need more information about Sui Rust SDK and available API methods it provides.

### Install

This project assumes `Rust` and `Cargo` are installed, and that there is an 
available internet connection. Please refer to the 
[Rust](https://www.rust-lang.org/tools/install) and 
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) 
documentation for the installation instructions.

- Clone the project:
```bash
git clone https://github.com/roman1e2f5p8s/sui-shared-objects.git
cd sui-shared-objects
```

- Build the project:
```bash
cargo build --release
```

## Usage
Building the project shall create three executable files:
1. `query-txs`;
2. `metrcis`;
3. `query-obj`.

> [!TIP]
> On UNIX-like systems, these can be executed using `./target/release/<NAME>`,
where `<NAME>` is one of the three executables listed above.

> [!TIP]
On Windows, these can be executed using `.\target\release\<NAME>.exe`, 
where `<NAME>` is one of the three executables listed above.

A more detailed description of how to use these executables and what they do 
are given in the next sub-sections.

### 1. `query-txs`
Use `query-txs` to query all the transactions (i.e., programmable transaction
blocks) for a given epoch, and pre-process them to save only the relevant
data we need for this analysis.

For example,
```bash
./target/release/query-txs --epoch=0
```
will query all transactions for epoch `0` and pre-process them to collect
only the relevant data for this analysis, as specified [here](
./data/README.md).

By default, the processed data will be saved in `data/workspace1/`, one file 
per epoch. You can create another workspace using the `--workspace` command 
line argument for `query-txs`.

For more information and all command line arguments, use `--help`:
```bash
./target/release/query-txs --help
```

See [this specification](./data/README.md) for the description and explanation 
of which data about Sui transactions `query-txs` collects and stores.

### 2. `metrics`
Use `metrics` to calculate [metrics](#metrics) of interest and obtain a set
of all shared object IDs for further analysis.

Specifically, executing
```bash
./target/release/metrics
```
will calculate metrics for all epochs the data is collected for using 
`query-txs`, collect IDs of all shared objects, and store them in `json` files 
whose structures are described [here](./results/README.md)

By default, the metrics and shared object IDs will be saved in 
`results/workspace1/`. The workspace must be the same as specified in 
`query-txs` and can be set using `--workspace` command line argument 
for `metrics`.

For more information and other command line arguments, use `--help`:
```bash
./target/release/metrics --help
```

### 3. `query-obj`
TODO

## Metrics
The following metrics are defined and calculated:
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
