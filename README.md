<p align="center">
    <a href="https://github.com/FuelLabs/sway-standard-implementations/actions/workflows/ci.yml" alt="CI">
        <img src="https://github.com/FuelLabs/sway-standard-implementations/actions/workflows/ci.yml/badge.svg" />
    </a>
    <a href="./LICENSE" alt="forc">
        <img src="https://img.shields.io/github/license/FuelLabs/sway-libs" />
    </a>
    <a href="https://discord.gg/xfpK4Pe">
        <img src="https://img.shields.io/discord/732892373507375164?color=6A7EC2&logo=discord&logoColor=ffffff&labelColor=6A7EC2&label=Discord" />
    </a>
</p>

# Sway Standard Implementations

## Overview

The purpose of this repository is to contain production ready implementations of the [sway standards](https://github.com/FuelLabs/sway-standards).

## Implementations

#### SRC-14: Owned Proxy Contract

- [Owned Proxy Contract](./src14/owned_proxy/) is an opinionated implementation of the [extended SRC-14 standard](https://docs.fuel.network/docs/sway-standards/src-14-simple-upgradeable-proxies/). It utilises the [Upgradability library from sway-libs](https://github.com/FuelLabs/sway-libs) and includes initialization functionality that allows for secure ownership upon deployment.

## Tests

To run the tests of the implementation projects; make sure you are in the source directory of this repository `sway-standard-implementations/<you are here>`.

Run the sdk-harness tests:

```bash
cd tests && forc test --release --locked && cargo test
```

> **NOTE:**
> This may take a while depending on your hardware, future improvements to Sway will decrease build times. After this has been run once, individual test projects may be built on their own to save time.

> **NOTE:**
> All projects currently use `forc v0.60.0`, `fuels-rs v0.63.0` and `fuel-core v0.27.0`. Versions are locked via `fuel-toolchain.toml` files.
