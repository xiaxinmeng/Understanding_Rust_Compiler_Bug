asm
...

        ; Function prologue (save stack pointer and reserve space)
        push    rbp
        mov     rbp, rsp
        push    rbx
        sub     rsp, 24
        ; This sets the internal ptr to the (later) heap allocated data to 4 (The pointer is set to the alignment of the type initially if the vector is empty.)
        mov     qword ptr [rbp - 32], 4
        ; And this sets the both the capacity and length to 0 using vector instructions.
        xorps   xmm0, xmm0
        movups  xmmword ptr [rbp - 24], xmm0

        ; Load pointer to the RawVec to pass to the double function (I think)
        lea     rdi, [rbp - 32]
        ; Call to capacity doubling function.
        call    <alloc::raw_vec::RawVec<T, A>>::double
        ; Load ptr to heap memory into %rdi
        mov     rdi, qword ptr [rbp - 32]
        ; Load length into %rax
        mov     rax, qword ptr [rbp - 16]
        ; Set first element to 0 (I suppose it could be possible to avoid doing the 4*rax bit here since the length should always be 0 at this point.)
        mov     dword ptr [rdi + 4*rax], 0
        ; Increment length
        inc     rax
        ; Store the length back to the length field in the vector instance
        mov     qword ptr [rbp - 16], rax
        ; Jump to bounds check panic if incrementing length wraps to zero(I think)
        je      .LBB3_2
        ; Load value of the first element into %ebx
        mov     ebx, dword ptr [rdi]
        ; Load the capacity into %rsi
        mov     rsi, qword ptr [rbp - 24]
        ; Check if capacity == 0
        test    rsi, rsi
        ; Jump to end and skip deallocation if the capacity of the vector was 0.
        je      .LBB3_6
        ; Set up arguments to rust_dealloc (I think)
        shl     rsi, 2
        mov     edx, 4
        ; Deallocate vector memory
        call    __rust_dealloc@PLT
.LBB3_6:
        ; Load value from %eax into return register
        mov     eax, ebx
        ; Function epilogue
        add     rsp, 24
        pop     rbx
        pop     rbp
        ret
...

