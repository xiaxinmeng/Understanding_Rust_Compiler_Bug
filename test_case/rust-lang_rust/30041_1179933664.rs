asm
bad1:
        cmp     byte ptr [rdi], 0
        je      .LBB1_2
        mov     byte ptr [rdi], 1
.LBB1_2:
        ret
