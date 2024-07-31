MEMORY
{
  RAM : ORIGIN = 0x80000, LENGTH = 256K
}

SECTIONS
{
  .text : {
    *(.text._start)
    *(.text*)
  } > RAM

  .bss : {
    *(.bss*)
  } > RAM

  .heap (NOLOAD) : {
    _heap_start = .;
    . = . + 0x10000;  // 64K for heap
    _heap_end = .;
  } > RAM

  .stack (NOLOAD) : {
    _stack_start = .;
    . = . + 0x2000;  // 8K for stack
    _stack_end = .;
  } > RAM
}
