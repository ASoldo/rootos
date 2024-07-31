![image](https://github.com/user-attachments/assets/2d073947-5282-44db-8c06-6c64d32c01d4)

# rootos

`rootos` is a minimal bare-metal OS kernel for AArch64, written in Rust.

## Prerequisites

- Rust and Cargo: [Install Rust](https://www.rustup.rs/)
- QEMU: [Install QEMU](https://www.qemu.org/download/)

## Building the Kernel

```sh
cargo build -Z build-std=core,compiler_builtins,alloc --target aarch64-unknown-none.json
```

## Preview the objdump

```sh
llvm-objdump -d target/aarch64-unknown-none/debug/rootos
```

## Running in QEMU

```sh
./run_rootos.sh
```

## Debugging with GDB

In a separate terminal, start GDB:

```sh
rust-gdb target/aarch64-unknown-none/debug/rootos
```

Connect GDB to the QEMU GDB server:

```sh
(gdb) target remote :1234
```

Use GDB commands to step through and debug the code:

- si: Step one instruction exactly.
- next: Step program, proceeding through subroutine calls.
- nexti: Step one instruction, but proceed through subroutine calls.
- continue: Continue program execution.
