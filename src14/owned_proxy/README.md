# SRC-14: Owned Proxy Contract

- The [Owned Proxy Contract](./contract/src/main.sw) is an opinionated implementation of the [extended SRC-14 standard](https://docs.fuel.network/docs/sway-standards/src-14-simple-upgradeable-proxies/). It utilises the [Upgradability library from sway-libs](https://github.com/FuelLabs/sway-libs) and includes initialization functionality that allows for secure ownership upon deployment.

## Usage instructions

Clone the repository and move into it:

```bash
git clone -b master https://github.com/FuelLabs/sway-standard-implementations.git && cd sway-standard-implementations
```

### Rust scripts

> **Warning:** These scripts are temporary. Proxy contract deployment and interactions will be integrated into `Forc` in the near future.

A suite of Rust scripts are included to assist in the usage of the contract. In order to use them navigate to the [owned-proxy/scripts](./scripts/) directory. From `sway-standard-implementations/<you are here>`:

```bash
cd src14/owned_proxy/scripts
```

#### Deploy and initialize

As described in the [SRC-14 standard](https://docs.fuel.network/docs/sway-standards/src-14-simple-upgradeable-proxies/); this proxy contract implementation works via The FuelVM's LDC instruction that is used by Sway's `std::execution::run_external` functionality to execute instructions from a target contract while retaining the proxy contract's storage context. As such a target contract must be deployed to be used alongside this proxy contract. There are two options to achieve this:

- The target contract can be deployed _before_ the proxy contract; in this case set the target contract's ID as the `<INITIAL_TARGET>` when deploying the proxy contract.
- The target contract can be deployed _after_ the proxy contract; in this case set the target contract via the `set_proxy_target` method, a script for which is provided [below](#updating-the-target-contract).

To deploy and initialize the proxy contract the `deploy_and_init` script is available. It will use the arguments `--initial-target` and `--initial-owner` as configurables in the proxy contract. From `sway-standard-implementations/src14/owned_proxy/scripts/<you are here>`:

```bash
SIGNING_KEY=<SIGNING_KEY> cargo run -r --bin deploy_and_init -- --initial-target <INITIAL_TARGET> --initial-owner <INITIAL_OWNER> --provider-url <PROVIDER_URL>
```

> **Note:** The optional flag `--provider-url <PROVIDER_URL>` sets the URL of the provider to be used in the script. If not manually set, it defaults to `127.0.0.1:4000` which is the default `fuel-core` URL.
> **Note:** There is also the optional flag `--signing-key <SIGNING_KEY>` which can be used instead of the environment variable `SIGNING_KEY`. However use of the environment variable `SIGNING_KEY` is preferred.

#### Updating the proxy owner

To update the proxy contract's owner the `set_proxy_owner` script is available. It will use the argument `--new-owner` as the new owner in the proxy contract. From `sway-standard-implementations/src14/owned_proxy/scripts/<you are here>`:

```bash
SIGNING_KEY=<SIGNING_KEY> cargo run -r --bin set_proxy_owner -- --proxy-contract-id <PROXY_CONTRACT_ID> --new-owner <NEW_OWNER> --provider-url <PROVIDER_URL>
```

> **Note:** The optional flag `--provider-url <PROVIDER_URL>` sets the URL of the provider to be used in the script. If not manually set, it defaults to `127.0.0.1:4000` which is the default `fuel-core` URL.
> **Note:** There is also the optional flag `--signing-key <SIGNING_KEY>` which can be used instead of the environment variable `SIGNING_KEY`. However use of the environment variable `SIGNING_KEY` is preferred.

#### Updating the target contract

To update the proxy contract's target the `set_proxy_target` script is available. It will use the argument `--new-target-id` as the new target contract in the proxy contract. From `sway-standard-implementations/src14/owned_proxy/scripts/<you are here>`:

```bash
SIGNING_KEY=<SIGNING_KEY> cargo run -r --bin set_proxy_target -- --proxy-contract-id <PROXY_CONTRACT_ID> --new-target-id <NEW_TARGET_ID> --provider-url <PROVIDER_URL>
```

> **Note:** The optional flag `--provider-url <PROVIDER_URL>` sets the URL of the provider to be used in the script. If not manually set, it defaults to `127.0.0.1:4000` which is the default `fuel-core` URL.
> **Note:** There is also the optional flag `--signing-key <SIGNING_KEY>` which can be used instead of the environment variable `SIGNING_KEY`. However use of the environment variable `SIGNING_KEY` is preferred.
