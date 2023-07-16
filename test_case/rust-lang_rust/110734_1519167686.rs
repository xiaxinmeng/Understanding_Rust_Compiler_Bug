asm
rust_test::exact_size:

    push r15
    push r14
    push rbx
    sub rsp, 64

    mov edi, 12
    mov esi, 4
    call qword ptr [rip + __rust_alloc@GOTPCREL]

    test rax, rax
    je .LBB0_6

    movabs rcx, 8589934593
    mov qword ptr [rax], rcx
    mov dword ptr [rax + 8], 3

    mov rcx, rax
    add rcx, 12

    mov qword ptr [rsp + 8], 3
    mov qword ptr [rsp + 16], rax
    mov qword ptr [rsp + 24], rcx
    mov qword ptr [rsp + 32], rax
    lea rax, [rsp + 8]

    #APP
    #NO_APP

    mov r15, qword ptr [rsp + 8]

    mov rsi, qword ptr [rsp + 16]

    mov rbx, qword ptr [rsp + 24]

    mov r14, qword ptr [rsp + 32]

    sub rbx, rsi

    cmp rsi, r14

    je .LBB0_3

    mov rdi, r14
    mov rdx, rbx
    call qword ptr [rip + memmove@GOTPCREL]

.LBB0_3:
    shr rbx, 2

    mov qword ptr [rsp + 40], r15
    mov qword ptr [rsp + 48], r14
    mov qword ptr [rsp + 56], rbx
    lea rax, [rsp + 40]
    #APP
    #NO_APP

    mov rsi, qword ptr [rsp + 40]

    test rsi, rsi
    je .LBB0_5

    mov rdi, qword ptr [rsp + 48]

    shl rsi, 2

    mov edx, 4
    call qword ptr [rip + __rust_dealloc@GOTPCREL]

.LBB0_5:
    add rsp, 64
    pop rbx
    pop r14
    pop r15
    ret
.LBB0_6:

    mov edi, 12
    mov esi, 4
    call qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
    ud2
