# rootos

`rootos` is a minimal bare-metal OS kernel for AArch64, written in Rust.

## Prerequisites

- Rust and Cargo: [Install Rust](https://www.rustup.rs/)
- QEMU: [Install QEMU](https://www.qemu.org/download/)

## Building the Kernel

```sh
cargo build -Z build-std=core,compiler_builtins,alloc --target aarch64-unknown-none.json
```

## Running in QEMU

```sh
./run_qemu.sh
```
