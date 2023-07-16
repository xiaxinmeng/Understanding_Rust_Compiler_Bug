asm
example::rotate_right1_fastest:
        movzx   eax, sil
        mov     al, byte ptr [rdi + rax]
        mov     cl, byte ptr [rdi + 4]
        cmp     sil, 5
        mov     rdx, rdi
        sbb     rdx, 0
        mov     byte ptr [rdx + 5], cl
        mov     cl, byte ptr [rdi + 3]
        cmp     sil, 4
        mov     rdx, rdi
        sbb     rdx, 0
        mov     byte ptr [rdx + 4], cl
        mov     cl, byte ptr [rdi + 2]
        cmp     sil, 3
        mov     rdx, rdi
        sbb     rdx, 0
        mov     byte ptr [rdx + 3], cl
        cmp     sil, 2
        mov     rcx, rdi
        sbb     rcx, 0
        mov     dl, byte ptr [rdi + 1]
        mov     byte ptr [rcx + 2], dl
        mov     cl, byte ptr [rdi]
        cmp     sil, 1
        mov     rdx, rdi
        sbb     rdx, -1
        mov     byte ptr [rdx], cl
        mov     byte ptr [rdi], al
        ret
