asm
example::compress:
        addi    sp, sp, -416 # 416 bytes of stack usage!
        … # <skipped function prelude>
        lui     a0, 16 # Two instructions to load the constant from K32
        addi    a0, a0, -256
        sw      a0, 260(sp) # Store it on the stack
        lui     a0, 272547 # Rince and repeat, for all 64 entries
        addi    a0, a0, -104
        sw      a0, 256(sp)
       …
