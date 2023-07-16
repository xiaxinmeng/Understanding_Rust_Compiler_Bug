asm
LAB_COND_TRUE:
    # some calls to alloc::fmt::format::format_inner
    ldr q0, [x25] # move the String...
    str q0, [sp, 0xb0] # ... into the destination place
    b LAB_BOMB
LAB_COND_FALSE:
    str xzr, [sp, 0xb8]
    b LAB_BOMB
    # ...
LAB_BOMB:
    add x12, sp, 0xb0 # unconditionally take pointer to Option<String>
    str x12, [sp, 0x1b0] # and put it into the "Struct"
