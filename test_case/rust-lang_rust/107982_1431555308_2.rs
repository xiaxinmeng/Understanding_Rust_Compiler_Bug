asm
LAB_COND_TRUE:
    # some calls to alloc::fmt::format::format_inner
    ldr q0, [x25] # move the String...
    str q0, [sp, 0xb0] # ... into the destination place
    # ...
    ldr x8, [sp, 0xb8]
    add x9, sp, 0xb0
    cmp x8, 0x0
    csel x11, xzr, x9, eq # I believe it is checking if String is empty
    b LAB_GOOD
LAB_COND_FALSE:
    mov x11, 0x0
    str xzr, [sp, 0xb8]
    b LAB_GOOD
    # ...
LAB_GOOD:
    str x11, [sp, 0x1b0] # Put it into the "Struct"
