asm
example::qux:
        mov     rax, rdi
        xor     ecx, ecx
        cmp     qword ptr [rsi], 1
        sete    cl
        mov     rdx, qword ptr [rsi + 24]
        mov     qword ptr [rdi + 24], rdx
        movups  xmm0, xmmword ptr [rsi + 8]
        movups  xmmword ptr [rdi + 8], xmm0
        mov     qword ptr [rdi], rcx
        ret

; Provided for comparison; note that defining `qux` and `bar` at the
; same time will cause `qux` to be optimized out with `-C opt-level=3`
example::bar:
        mov     rax, rdi
        xor     ecx, ecx
        cmp     qword ptr [rsi], 1
        sete    cl
        mov     rdx, qword ptr [rsi + 24]
        mov     qword ptr [rdi + 24], rdx
        movups  xmm0, xmmword ptr [rsi + 8]
        movups  xmmword ptr [rdi + 8], xmm0
        mov     qword ptr [rdi], rcx
        ret
