ENTRY(kernel_entry)

SECTIONS
{
  . = 0x1000;

  .text :
  {
    KEEP(*(.text.kernel_entry))  /* ensure it comes first */
    *(.text*)                   /* all other text after */
  }

  .rodata : { *(.rodata*) }
  .data   : { *(.data*) }
  .bss    : { *(.bss*) }
}
