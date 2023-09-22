# Sui Shared Object Density

The [sui-shared-object-density](https://github.com/roman1e2f5p8s/sui-shared-object-density) 
is a Rust-based project that provides convenient tools to estimate and visualize the density 
of transactions involving shared objects (and other metrics) on the 
[Sui network](https://sui.io/) using the [Sui Rust SDK](https://docs.sui.io/build/rust-sdk). 
Hereafter, the **density** means the ratio of the number of transactions touching shared 
objects to the total number of transactions for a given time interval.

## Table of Contents

- [Background](#background)
- [Requirements](#requirements)
    - [Dependencies](#dependencies)
- [Getting Started](#getting-started)
    - [Install](#install)
- [Query](#query-usage)
- [Plot](#plot-usage)
- [Examples](#examples)
- [Contribute](#contribute)
- [License](#license)

## Background

Sui is a layer-1 smart contract platform utilizing an object-centric data model: the basic unit 
of storage in Sui is an **object**. The Sui ledger, therefore, stores a collection of 
programmable objects, each with a globally unique ID.

From the ownership point of view, there are two types of Sui objects:
- **Owned objects**: owned by an address and can be used only by transactions signed by that 
owner address at a time. 
- **Shared objects**: no specific owner; anyone can read or write this object. 

Mutable owned objects are **single-writers**, and thus, transactions involving only owned objects 
may bypass the consensus protocols in Sui. Mutable shared objects (**multi-writers**), however, 
require consensus to sequence (order) reads and writes.

### Motivation

Owned objects are the most common case in Sui. Additionally, according to the 
[Sui documentation](https://docs.sui.io/learn/how-sui-works#transactions-on-single-owner-objects), 
many transactions (e.g., asset transfers, NFT minting, and smart contract publishing) 
can be realized involving only owned objects. See this 
[list](https://docs.sui.io/learn/single-writer-apps), provided by Sui, of potential 
single-writer real-world applications.

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
