asm
example::clone_slice_old:
        ; ...
        mov     qword ptr [rsp], r14
        ; ...
        mov     rax, qword ptr [rsp]
        ; ...
