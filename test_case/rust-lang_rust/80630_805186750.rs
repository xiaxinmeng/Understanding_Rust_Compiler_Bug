
example::exec:  << The start of exec() function in rust
        sub     rsp, 8
        mov     dword ptr [rsp + 4], edi
.LBB2_1: << The block that implements the 'switch' 
        mov     eax, dword ptr [rsp + 4]
        mov     dword ptr [rsp], eax
        test    eax, eax
        je      .LBB2_3
        jmp     .LBB2_9
.LBB2_9: 
        mov     eax, dword ptr [rsp]
        sub     eax, 1
        je      .LBB2_5
        jmp     .LBB2_10
.LBB2_10:
        mov     eax, dword ptr [rsp]
        sub     eax, 2
        je      .LBB2_7
        jmp     .LBB2_2
.LBB2_2:
        pop     rax
        ret
.LBB2_3: << case 0
        lea     rdi, [rip + .L__unnamed_2]
        mov     esi, 1
        call    qword ptr [rip + example::print@GOTPCREL]
        mov     dword ptr [rsp + 4], 2 << explicit store to stack: s = 2
        jmp     .LBB2_1 << BAD: jumps back to the 'switch', but s value is known
.LBB2_5: << case 1
        lea     rdi, [rip + .L__unnamed_3]
        mov     esi, 1
        call    qword ptr [rip + example::print@GOTPCREL]
        mov     dword ptr [rsp + 4], 3 << explicit store to stack: s = 3
        jmp     .LBB2_1 << BAD: jumps back to the 'switch' , but s value is known
.LBB2_7: << case 2
        lea     rdi, [rip + .L__unnamed_4]
        mov     esi, 1
        call    qword ptr [rip + example::print@GOTPCREL]
        mov     dword ptr [rsp + 4], 1 << explicit store to stack: s = 1
        jmp     .LBB2_1 << BAD: jumps back to the 'switch' , but s value is known
