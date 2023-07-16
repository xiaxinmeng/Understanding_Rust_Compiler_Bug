asm
foo1a:
    cmp     dil, sil
    jne     .LBB0_6
    mov     rcx, rsi
    shr     rcx, 8
    mov     rdx, rdi
    shr     rdx, 8
    xor     eax, eax
    cmp     dl, cl
    jne     .LBB0_7
    mov     rcx, rsi
    shr     rcx, 16
    mov     rdx, rdi
    shr     rdx, 16
    cmp     dl, cl
    jne     .LBB0_7
    mov     rcx, rdi
    shr     rcx, 24
    mov     rdx, rsi
    shr     rdx, 24
    xor     eax, eax
    cmp     cl, dl
    jne     .LBB0_7
    mov     rcx, rdi
    shr     rcx, 32
    mov     rdx, rsi
    shr     rdx, 32
    cmp     cl, dl
    jne     .LBB0_7
    mov     r8, rdi
    shr     r8, 40
    mov     rcx, rdi
    shr     rcx, 48
    shr     rdi, 56
    mov     rdx, rsi
    shr     rdx, 40
    mov     rax, rsi
    shr     rax, 48
    shr     rsi, 56
    xor     dl, r8b
    xor     al, cl
    or      al, dl
    xor     sil, dil
    or      sil, al
    sete    al
    ret
.LBB0_6:
    xor     eax, eax
.LBB0_7:
    ret


foo1b:
    mov     al, byte ptr [rdi]
    cmp     al, byte ptr [rsi]
    jne     .LBB1_1
    mov     eax, dword ptr [rdi + 1]
    mov     ecx, dword ptr [rdi + 4]
    xor     eax, dword ptr [rsi + 1]
    xor     ecx, dword ptr [rsi + 4]
    or      ecx, eax
    sete    al
    ret
.LBB1_1:
    xor     eax, eax
    ret


foo2a:
    cmp     rdi, rsi
    sete    al
    ret


foo2b:
    mov     rax, qword ptr [rdi]
    cmp     rax, qword ptr [rsi]
    sete    al
    ret
