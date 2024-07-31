.section .text._start
.global _start
.type _start, @function
_start:
    # Initialize the stack
    adr x7, _stack
    mov x8, #8192
    add x7, x7, x8
    mov sp, x7

    # Enable floating point bits FPEN
    mrs x7, cpacr_el1
    mov x8, #(3 << 20)
    orr x7, x8, x7
    msr cpacr_el1, x7

    # Call into main
    bl main

    # Infinite loop to prevent returning from _start
1:  b 1b

.section .bss
_stack:
    .skip 8192
