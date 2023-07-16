asm
rust_test::var_size:

    push rbx
    sub rsp, 32

    mov edi, 12
    mov esi, 4
    call qword ptr [rip + __rust_alloc@GOTPCREL]

    test rax, rax
    je .LBB1_12

    movabs rcx, 8589934593
    mov qword ptr [rax], rcx
    mov dword ptr [rax + 8], 3

    mov rcx, rax
    add rcx, 12

    mov qword ptr [rsp], 3
    mov qword ptr [rsp + 8], rax
    mov qword ptr [rsp + 16], rcx
    mov qword ptr [rsp + 24], rax
    mov rax, rsp

    #APP
    #NO_APP

    mov rcx, qword ptr [rsp]

    mov r8, qword ptr [rsp + 8]

    mov rsi, qword ptr [rsp + 16]

    mov rdx, qword ptr [rsp + 24]

    mov rdi, rdx

    cmp r8, rsi
    je .LBB1_9

    mov r10, rsi
    sub r10, r8
    add r10, -4
    cmp r10, 28
    jb .LBB1_3

    mov rdi, rdx

    sub rdi, r8
    cmp rdi, 32
    jb .LBB1_3

    shr r10, 2
    inc r10
    mov r11, r10
    and r11, -8
    lea rdi, [rdx + 4*r11]
    lea r9, [r8 + 4*r11]
    xor ebx, ebx

.LBB1_6:
    movups xmm0, xmmword ptr [r8 + 4*rbx]
    movups xmm1, xmmword ptr [r8 + 4*rbx + 16]

    movups xmmword ptr [rdx + 4*rbx], xmm0
    movups xmmword ptr [rdx + 4*rbx + 16], xmm1
    add rbx, 8
    cmp r11, rbx
    jne .LBB1_6

    cmp r10, r11
    jne .LBB1_8
    jmp .LBB1_9

.LBB1_3:
    mov rdi, rdx
    mov r9, r8

.LBB1_8:
    mov r8d, dword ptr [r9]

    add r9, 4

    mov dword ptr [rdi], r8d

    add rdi, 4

    cmp r9, rsi
    jne .LBB1_8

.LBB1_9:
    sub rdi, rdx

    shr rdi, 2

    mov qword ptr [rsp], rcx
    mov qword ptr [rsp + 8], rdx
    mov qword ptr [rsp + 16], rdi
    #APP
    #NO_APP

    mov rsi, qword ptr [rsp]

    test rsi, rsi
    je .LBB1_11

    mov rdi, qword ptr [rsp + 8]

    shl rsi, 2

    mov edx, 4

    call qword ptr [rip + __rust_dealloc@GOTPCREL]

.LBB1_11:
    add rsp, 32
    pop rbx
    ret
.LBB1_12:

    mov edi, 12
    mov esi, 4
    call qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
    ud2
