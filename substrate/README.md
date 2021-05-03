## Rust Setup

Follow [this link](https://www.rust-lang.org/tools/install) to setup Rust in your system.

## Build

Run this command inside the provenance-usecase/substrate folder to build the binary file.

```sh
cargo build --release
```

## Run

Use this command to start the substrate node.

```sh
./target/release/provenance-substrate --dev --tmp --alice --ws-external --rpc-external
```