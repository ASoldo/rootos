#!/bin/bash
qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a76 \
    -kernel target/aarch64-unknown-none/debug/rootos \
    -serial mon:stdio \
