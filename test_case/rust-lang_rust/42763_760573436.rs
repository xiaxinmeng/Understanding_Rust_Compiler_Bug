asm
example::main:
        push    r14
        push    rbx
        sub     rsp, 872
        mov     qword ptr [rsp + 24], 5
        mov     qword ptr [rsp + 32], 5
        mov     qword ptr [rsp + 40], 5
        mov     qword ptr [rsp + 48], 5
        mov     qword ptr [rsp + 56], 5
        mov     qword ptr [rsp + 64], 5
        mov     qword ptr [rsp + 72], 5
        mov     qword ptr [rsp + 80], 5
        mov     qword ptr [rsp + 88], 5
        mov     qword ptr [rsp + 96], 5
        mov     qword ptr [rsp + 104], 5
        mov     qword ptr [rsp + 112], 5
        mov     qword ptr [rsp + 120], 5
        mov     qword ptr [rsp + 128], 5
        mov     qword ptr [rsp + 136], 5
        mov     qword ptr [rsp + 144], 5
        mov     qword ptr [rsp + 152], 5
        mov     qword ptr [rsp + 160], 5
        mov     qword ptr [rsp + 168], 5
        mov     qword ptr [rsp + 176], 5
        mov     qword ptr [rsp + 184], 5
        mov     qword ptr [rsp + 192], 5
        mov     qword ptr [rsp + 200], 5
        mov     qword ptr [rsp + 208], 5
        mov     qword ptr [rsp + 216], 5
        mov     qword ptr [rsp + 224], 5
        mov     qword ptr [rsp + 232], 5
        mov     qword ptr [rsp + 240], 5
        mov     qword ptr [rsp + 248], 5
        mov     qword ptr [rsp + 256], 5
        mov     qword ptr [rsp + 264], 5
        mov     qword ptr [rsp + 272], 5
        mov     qword ptr [rsp + 280], 5
        mov     qword ptr [rsp + 288], 5
        mov     qword ptr [rsp + 296], 5
        mov     qword ptr [rsp + 304], 5
        mov     qword ptr [rsp + 312], 5
        mov     qword ptr [rsp + 320], 5
        mov     qword ptr [rsp + 328], 5
        mov     qword ptr [rsp + 336], 5
        mov     qword ptr [rsp + 344], 5
        mov     qword ptr [rsp + 352], 5
        mov     qword ptr [rsp + 360], 5
        mov     qword ptr [rsp + 368], 5
        mov     qword ptr [rsp + 376], 5
        mov     qword ptr [rsp + 384], 5
        mov     qword ptr [rsp + 392], 5
        mov     qword ptr [rsp + 400], 5
        mov     qword ptr [rsp + 408], 5
        mov     qword ptr [rsp + 416], 5
        lea     r14, [rsp + 472]
        lea     rsi, [rsp + 24]
        mov     rbx, qword ptr [rip + memcpy@GOTPCREL]
        mov     edx, 400
        mov     rdi, r14
        call    rbx
        lea     rdi, [rsp + 24]
        mov     edx, 400
        mov     rsi, r14
        call    rbx
        lea     rax, [rsp + 104]
        mov     qword ptr [rsp + 8], rax
        mov     rax, qword ptr [rip + core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt@GOTPCREL]
        mov     qword ptr [rsp + 16], rax
        lea     rax, [rip + .L__unnamed_1]
        mov     qword ptr [rsp + 424], rax
        mov     qword ptr [rsp + 432], 2
        mov     qword ptr [rsp + 440], 0
        lea     rax, [rsp + 8]
        mov     qword ptr [rsp + 456], rax
        mov     qword ptr [rsp + 464], 1
        lea     rdi, [rsp + 424]
        call    qword ptr [rip + std::io::stdio::_print@GOTPCREL]
        add     rsp, 872
        pop     rbx
        pop     r14
        ret
