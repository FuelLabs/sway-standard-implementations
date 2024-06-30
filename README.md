# sway-standard-implementations

## Running Tests

Make sure you are in the source directory of this repository `sway-standard-implementations/<you are here>`.

Run the sdk-harness tests:

```bash
forc test --path tests --release --locked && cargo test --manifest-path tests/Cargo.toml
```

> **NOTE:**
> This may take a while depending on your hardware, future improvements to Sway will decrease build times. After this has been run once, individual test projects may be built on their own to save time.

> **NOTE:**
> All projects currently use `forc v0.60.0`, `fuels-rs v0.63.0` and `fuel-core v0.27.0`. Versions are locked via `fuel-toolchain.toml` files.
