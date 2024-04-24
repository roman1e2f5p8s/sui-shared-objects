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
> [!IMPORTANT]
> Use `query-txs` to query all the transactions (i.e., programmable transaction
> blocks) for a given epoch, and pre-process them to save only the relevant
> data we need for this analysis.

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
> [!IMPORTANT]
> Use `metrics` to calculate [metrics](#metrics) of interest and obtain a set
> of all shared object IDs for further analysis.

Specifically, executing
```bash
./target/release/metrics
```
will calculate metrics for all epochs the data is collected for using 
`query-txs`, collect IDs of all shared objects, and store them in `json` files 
whose structures are described [here](./results/README.md).

> [!NOTE]
> By default, the metrics and shared object IDs will be saved in 
> `results/workspace1/`. The workspace must be the same as specified in 
> `query-txs` and can be changed using `--workspace` command line argument 
> for `metrics`.

For more information and other command line arguments, use `--help`:
```bash
./target/release/metrics --help
```

### 3. `query-obj`
> [!IMPORTANT]
> Use `query-obj` to obtain information about a set of collected shared objects
> (returned by [`metrics`](#2-metrics)) and packages that implement them:

```bash
./target/release/query-obj
```

> [!NOTE]
> By default, the data shared objects and packages will be saved in 
> `results/workspace1/`. The workspace must be the same as specified in 
> `metrics` and can be changed using `--workspace` command line argument 
> for `query-obj`.

For more information and all command line arguments, use `--help`:
```bash
./target/release/query-obj --help
```

See [this specification](./results/README.md) for the description and 
explanation data about shared objects and packages `query-obj` collects and 
stores.

## Metrics
Recall the following concepts from Sui:
- **Epoch**: In Sui, each epoch takes approximately 24 hours.
- **Checkpoint**: A checkpoint (also called sequence number) in Sui changes 
approximately every 2-3 second.

We also need to define auxiliary concepts:
- **Interval**: An interval is a period of time expressed in the number of 
checkpoints.
- **Contention**: Contention is a situation when multiple transactions touch
the same shared object at the same time, i.e., concurrently access that shared 
object.
- **Shared-object transaction**: A shared-object transaction has at least one 
shared object in its inputs.

The following metrics are defined and calculated:
- **Density**: The density is the ratio of the number of shared-object 
transactions to the number of all transactions. The density is a number 
between 0 and 1; the higher the density, the more transactions operate on 
shared objects.
- **Contention degree**: The contention degree is the ratio of the number of 
shared-object transactions (within some interval) to the number of shared 
objects touched by those transactions (within the same interval). The 
contention degree is a number between 0 and ∞. A contention degree of 1 means 
that each shared-object transaction operates on a single different object, 
on average; values larger than 1 indicate multiple shared-object transactions 
contending for the same shared object; values smaller than 1 mean a transaction 
touches multiple shared objects, on average.
- **Contended fraction**: The contended fraction is the ratio of the number 
of shared objects (within some interval) touched by more than one transaction 
to the total number of shared objects (within the same interval). The 
contended fraction is a number between 0 and 1. The higher the contended 
fraction, the more shared objects are touched by more than one transaction.

We also calculate and plot the following simple metrics:
- **The total number of transactions** (per epoch).
- **Number of shared-objects transactions** (per epoch). 
- **Number of shared objects** (touched per epoch).

## Contribute
TODO
