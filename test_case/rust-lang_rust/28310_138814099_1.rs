 nasm
main:
    push    rbp
    mov rbp, rsp
    sub rsp, 32
    lea rax, [rbp - 8] ; first, remember that rax points to the Dropper

    ; begin the memset call
    mov esi, 29 ; this is what we're memsetting to
    mov ecx, 1
    mov edx, ecx ; set one byte
    mov byte ptr [rbp - 1], 61

    mov byte ptr [rbp - 8], -44 ; notice we set the drop flag here
    mov dil, byte ptr [rbp - 8]
    mov byte ptr [rbp - 9], dil
    mov r8, rax
    mov qword ptr [rbp - 24], rax ; r8 and [rbp - 24] are both pointing to the Dropper now
    mov rdi, r8 ; we're memsetting the Dropper, here
    call    memset@PLT   ; this memset

    mov r9b, byte ptr [rbp - 9]
    mov byte ptr [rbp - 10], r9b

    mov rdi, qword ptr [rbp - 24]
    call    _ZN7Dropper9drop.256317h2845f499b38a318cE

    add rsp, 32
    pop rbp
    ret
