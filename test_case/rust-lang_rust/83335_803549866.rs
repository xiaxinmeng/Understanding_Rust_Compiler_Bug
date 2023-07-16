
// Linker script
SECTIONS
{
  . = 0xffff000040080000;

  .text : {
    *(.text.boot)
    *(.text .text.* .gnu.linkonce.t*)
    . = ALIGN(4K);
  }

  .rodata : {
    *(.rodata .rodata.* .gnu.linkonce.r*)
    . = ALIGN(4K);
  }

  .data : {
    *(.data .data.* .gnu.linkonce.d*)
    . = ALIGN(4K);
  }

  .stack : {
    . = ALIGN(4K);
    *(.bss.stack)
  }

  .bss : {
    . = ALIGN(32);
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(4K);
  }

  /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
