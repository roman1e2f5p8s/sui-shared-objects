# Sui Shared Object Density

The [sui-shared-object-density](https://github.com/roman1e2f5p8s/sui-shared-object-density) 
is a Rust-based project that provides convenient tools to estimate the density of transactions 
involving shared objects on the [Sui network](https://sui.io/) using the 
[Sui Rust SDK](https://docs.sui.io/build/rust-sdk). Hereafter, the **density** is defined as 
a ratio of the number of transactions touching shared objects to the total number of 
transactions for a given time interval.

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
of storage in Sui is **object**. The Sui ledger therefore stores a collection of 
programmable objects, each with a globally unique ID.

From the ownership point of view, there are two types of Sui objects:
- **Owned objects**: This is the most common case. Upon creation in the code, such object can be 
transferred to an address. After the transfer, this object will be owned by that address. 
An object owned by an address can be used only by transactions signed by that owner address. 
Mutable owned objects are single-writers and should never undergo contention. Transactions
involving only owned objects will be called **simple transactions** and are causally ordered in Sui.
- **Shared objects**: An object can be shared, meaning that anyone can read or write this object. 
In contrast to mutable owned objects, shared objects require consensus to sequence reads 
and writes. Transactions involving shared objects will be called **complex transactions** and are
totally ordered by consensus in Sui.

The distinction of causal and total ordering enables massively parallel transaction execution 
in Sui. Sui programmers often have the choice to implement a particular use-case using 
shared objects, owned objects, or a combination. This choice can have implications for 
performance, security, and implementation complexity.


### Motivation
