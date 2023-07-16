asm
foo1:
        test    esi, esi
        jle     .LBB0_9
        test    edi, edi
        js      .LBB0_9
        cmp     edi, esi
        jge     .LBB0_9
        test    edi, edi
        je      .LBB0_4
        xor     ecx, ecx
        mov     r9d, 1
        mov     r8d, esi
.LBB0_8:
        mov     eax, r8d
        cdq
        idiv    edi
        mov     r8d, eax
        mov     eax, r9d
        imul    r8d, r9d
        sub     ecx, r8d
        mov     r10d, edi
        mov     r8d, edi
        mov     r9d, ecx
        mov     edi, edx
        mov     ecx, eax
        test    edx, edx
        jne     .LBB0_8
        cmp     r10d, 1
        jne     .LBB0_9
.LBB0_6:
        add     eax, esi
        cdq
        idiv    esi
        mov     eax, 1
        ret
.LBB0_4:
        xor     eax, eax
        mov     r10d, esi
        cmp     r10d, 1
        je      .LBB0_6
.LBB0_9:
        xor     eax, eax
        ret

foo2:
        sub     rsp, 8
        cmp     edi, esi
        jge     .LBB1_6
        test    edi, edi
        js      .LBB1_6
        test    esi, esi
        jle     .LBB1_6
        test    edi, edi
        je      .LBB1_4
        xor     ecx, ecx
        mov     r9d, 1
        mov     r8d, esi
.LBB1_8:
        mov     eax, r8d
        cdq
        idiv    edi
        mov     r8d, eax
        mov     eax, r9d
        imul    r8d, r9d
        sub     ecx, r8d
        mov     r10d, edi
        mov     r8d, edi
        mov     r9d, ecx
        mov     edi, edx
        mov     ecx, eax
        test    edx, edx
        jne     .LBB1_8
        cmp     r10d, 1
        jne     .LBB1_6
.LBB1_9:
        test    esi, esi
        je      .LBB1_13
        add     eax, esi
        cmp     esi, -1
        jne     .LBB1_12
        cmp     eax, -2147483648
        je      .LBB1_14
.LBB1_12:
        cdq
        idiv    esi
        mov     eax, 1
        pop     rcx
        ret
.LBB1_4:
        xor     eax, eax
        mov     r10d, esi
        cmp     r10d, 1
        je      .LBB1_9
.LBB1_6:
        xor     eax, eax
        pop     rcx
        ret
.LBB1_13:
        lea     rdi, [rip + str.0]
        lea     rdx, [rip + .L__unnamed_1]
        mov     esi, 57
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2
.LBB1_14:
        lea     rdi, [rip + str.1]
        lea     rdx, [rip + .L__unnamed_1]
        mov     esi, 48
        call    qword ptr [rip + core::panicking::panic@GOTPCREL]
        ud2

.L__unnamed_2:
        .ascii  "./example.rs"

str.0:
        .ascii  "attempt to calculate the remainder with a divisor of zero"

str.1:
        .ascii  "attempt to calculate the remainder with overflow"

.L__unnamed_1:
        .quad   .L__unnamed_2
        .asciz  "\f\000\000\000\000\000\000\000 \000\000\000\016\000\000"
