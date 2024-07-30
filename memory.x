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
    *(.bss.heap)
    *(.bss.stack)
    *(COMMON)
  } > RAM

  .heap : {
    *(.heap*)
  } > RAM
}
