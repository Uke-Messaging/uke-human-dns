# Uke Human DNS ink! Smart Contract

## Description

An ink! smart contract that maps cryptographic addresses to more human readable names, just like a DNS. Coined the "Human DNS", it essentially maps the hashes of unique, human readable ids to otherwise illegible addresses via a hash. With this verifiable mapping of addresses, client-side implementations can then look up other users and add them to their contacts, or write them a new message, or any other package of data in theory.

## Requirements

- Rust & Cargo
- ink! CLI

For an extended guide, please view: https://ink.substrate.io/getting-started/setup. This is required before running, compiling, or running tests for this repository.

## Building & Running Tests

To run the included unit tests, you can run

```sh
cargo +nightly contract test
```

To build the smart contract into a usable WASM executable, you can run

```sh
cargo +nightly contract build
```

## Deploying to Substrate Contract UI

Firstly, install the `substrate-contract-node` to your commandline using `cargo` :

```sh
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.17.0 --force --locked
```

Build the contract:

```sh
cargo +nightly contract build
```

Start your Substrate development node:

```sh
substrate-contracts-node --dev --tmp
```

Once it's started and you see blocks populating, navigate to https://contracts-ui.substrate.io/ and click the upper left and select `Local Node`.

You may now upload `uke_human_dns.contract` to the node via `Add New Contract` in the left pane. Click through the UI and upload the contract to the network.
