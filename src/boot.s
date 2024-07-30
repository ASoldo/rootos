.section .text._start
.global _start
.type _start, @function
_start:
    # Initialize the stack
    adr x7, _stack
    mov x8, #1024
    add x7, x7, x8
    mov sp, x7

    # Call into main
    bl main

    # Infinite loop to prevent returning from _start
1:  b 1b

.section .bss
_stack:
    .skip 4096
