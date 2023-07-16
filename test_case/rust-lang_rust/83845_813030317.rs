asm
        call    qword ptr [rip + std::io::stdio::_print@GOTPCREL]
        add     rsp, 96
        pop     rbx
        ret
        mov     rbx, rax
        mov     rdi, rsp
        call    core::ptr::drop_in_place<example::CompilerCheck<i32>>
        mov     rdi, rbx
        call    _Unwind_Resume@PLT
        ud2
