# SRC-14: Owned Proxy Contract

<p align="center">
<a href="https://crates.io/crates/forc/0.63.5" alt="forc">
        <img src="https://img.shields.io/badge/forc-v0.63.5-orange" />
    </a>
</p>

- The [Owned Proxy Contract](./contract/src/main.sw) is an opinionated implementation of the [extended SRC-14 standard](https://docs.fuel.network/docs/sway-standards/src-14-simple-upgradeable-proxies/). It utilises the [Upgradability library from sway-libs](https://github.com/FuelLabs/sway-libs) and includes initialization functionality that allows for secure ownership upon deployment.

## Recommended Usage

The recommended way to utilise the [Owned Proxy Contract](./contract/src/main.sw) is with [forc-deploy's proxy feature](https://docs.fuel.network/docs/forc/plugins/forc_client/#proxy-contracts). A guide on recommended usage of proxies and large contracts is available at: [How To Use Proxy contracts & large contract chunking](https://github.com/FuelLabs/proxy-chunks-minimal-example/tree/main).

## Manual utility scripts

This project does provide some [simple Rust scripts](./scripts/README.md) as examples for basic deployment and interaction functionality. However; this is NOT the recommended way of utilising the [Owned Proxy Contract](./contract/src/main.sw).
