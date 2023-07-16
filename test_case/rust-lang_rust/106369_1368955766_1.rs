asm
example::test_ptr_read:
        cmp     qword ptr [rdi], 0
        setne   al
        ret

example::test_deref:
        mov     al, 1
        ret

example::test_expected:
        mov     al, 1
        ret
