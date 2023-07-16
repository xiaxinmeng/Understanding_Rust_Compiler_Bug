asm
    ; copy word 0
        mov     rcx, qword ptr [rsp]
        mov     qword ptr [rdi], rcx
    ; copy words 1-2
        vmovups xmm0, xmmword ptr [rsp + 8]
        vmovups xmmword ptr [rdi + 8], xmm0
    ; copy word 3
        mov     rcx, qword ptr [rsp + 24]
        mov     qword ptr [rdi + 24], rcx
    ; copy word 2 again (??)
        mov     rcx, qword ptr [rsp + 16]
        mov     qword ptr [rdi + 16], rcx
    ; copy word 3 again (??)
        mov     rcx, qword ptr [rsp + 24]
        mov     qword ptr [rdi + 24], rcx
    ; copy words 4-5
        vmovups xmm0, xmmword ptr [rsp + 32]
        vmovups xmmword ptr [rdi + 32], xmm0
