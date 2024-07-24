# SRC-14: Owned Proxy Contract

- The [Owned Proxy Contract](./contract/src/main.sw) is an opinionated implementation of the [extended SRC-14 standard](https://docs.fuel.network/docs/sway-standards/src-14-simple-upgradeable-proxies/). It utilises the [Upgradability library from sway-libs](https://github.com/FuelLabs/sway-libs) and includes initialization functionality that allows for secure ownership upon deployment.

## Usage instructions

Clone the repository and move into it:

```bash
git clone -b master https://github.com/FuelLabs/sway-standard-implementations.git && cd sway-standard-implementations
```

### Rust scripts

A suite of Rust scripts are included to assist in the usage of the contract. In order to use them navigate to the [owned-proxy/scripts](./scripts/) directory. From `sway-standard-implementations/<you are here>`:

```bash
cd src14/owned_proxy/scripts
```

#### Deploy and initialize

To deploy and initialize the proxy contract the `deploy_and_init` script is available. It will use the arguments `--initial-target` and `--initial-owner` as configurables in the proxy contract. From `sway-standard-implementations/src14/owned_proxy/scripts/<you are here>`:

```bash
cargo run -r --bin deploy_and_init -- --signing-key <SIGNING_KEY> --initial-target <INITIAL_TARGET> --initial-owner <INITIAL_OWNER>
```

> **Note:** There is also the optional flag `--provider-url <PROVIDER_URL>` which sets the URL of the provider to be used in the script. If not manually set, it defaults to `testnet.fuel.network`.

#### Updating the proxy owner

To update the proxy contract's owner the `set_proxy_owner` script is available. It will use the argument `--new-owner` as the new owner in the proxy contract. From `sway-standard-implementations/src14/owned_proxy/scripts/<you are here>`:

```bash
cargo run -r --bin set_proxy_owner -- --signing-key <SIGNING_KEY> --proxy-contract-id <PROXY_CONTRACT_ID> --new-owner <NEW_OWNER>
```

> **Note:** There is also the optional flag `--provider-url <PROVIDER_URL>` which sets the URL of the provider to be used in the script. If not manually set, it defaults to `testnet.fuel.network`.

#### Updating the target contract

To update the proxy contract's target the `set_proxy_target` script is available. It will use the argument `--new-target-id` as the new target contract in the proxy contract. From `sway-standard-implementations/src14/owned_proxy/scripts/<you are here>`:

```bash
cargo run -r --bin set_proxy_target -- --signing-key <SIGNING_KEY> --proxy-contract-id <PROXY_CONTRACT_ID> --new-target-id <NEW_TARGET_ID>
```

> **Note:** There is also the optional flag `--provider-url <PROVIDER_URL>` which sets the URL of the provider to be used in the script. If not manually set, it defaults to `testnet.fuel.network`.
