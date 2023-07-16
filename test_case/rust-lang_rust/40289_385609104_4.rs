 ld
/* link.x */
EXTERN(STATIC); /* Forces the linker to look into libfoo.rlib */
/* In general this makes the linker continue looking into the inputs until it finds this symbol */

SECTIONS
{
    /* put STATIC in the .static linker section */
    .static : ALIGN(4)
    {
        KEEP(*(.rodata.STATIC));
    }
}
