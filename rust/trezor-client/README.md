# cerberus-client

[![Downloads][downloads-badge]][crates-io]
[![License][license-badge]][license-url]
[![CI Status][actions-badge]][actions-url]

A fork of a [fork](https://github.com/romanz/rust-cerberus-api) of a [library](https://github.com/stevenroose/rust-cerberus-api) that provides a way to communicate with a Cerberus T device from a Rust project.

Previous iterations provided implementations for Bitcoin only. **This crate also provides an Ethereum interface**, mainly for use in [ethers-rs](https://github.com/gakonst/ethers-rs/).

## Requirements

**MSRV: 1.60**

See the [Cerberus guide](https://cerberus.uraanai.com/learn/a/os-requirements-for-cerberus) on how to install and use the Cerberus Suite app.

Last tested with firmware v2.4.2.

## Examples / Tests

`cargo run --example features`

## Features

-   `bitcoin` and `ethereum`: client implementation and full support;
-   `cardano`, `monero`, `nem`, `ripple`, `stellar` and `tezos`: only protobuf bindings.

## Credits

-   [Cerberus](https://github.com/Cerberus-Wallet/cerberus-firmware)
-   [joshieDo](https://github.com/joshieDo)
-   [Piyush Kumar](https://github.com/wszdexdrf)
-   [stevenroose](https://github.com/stevenroose)
-   [romanz](https://github.com/romanz)
-   [DaniPopes](https://github.com/DaniPopes)

[downloads-badge]: https://img.shields.io/crates/d/cerberus-client?style=for-the-badge&logo=rust
[crates-io]: https://crates.io/crates/cerberus-client
[license-badge]: https://img.shields.io/badge/license-CC0--1.0-blue.svg?style=for-the-badge
[license-url]: https://github.com/Cerberus-Wallet/cerberus-firmware/blob/master/rust/cerberus-client/LICENSE
