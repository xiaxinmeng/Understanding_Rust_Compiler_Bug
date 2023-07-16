asm
core::ptr::write:
        ;; writes 42 to the address at ecx
        mov     byte ptr [ecx], 42
        ret
core::ptr::<impl *mut T>::add:
        ;; increments the address at ecx by 1
        lea     eax, [ecx + 1]
        ret
example::bar:
        sub     esp, 24
        ;; our union is at esp + 8, create ptr:
        lea     ecx, [esp + 8]
        ;; increment its address by 1
        call    core::ptr::<impl *mut T>::add
        mov     ecx, eax
        ;; writes 42 to the address (esp + 9)
        call    core::ptr::<impl *mut T>::write
        ;; now passing the union to foo begins
        ;; S.0 is put in `al` - padding be damned
        mov     al, byte ptr [esp + 8]
        ;; S.1 is put in `ecx`
        mov     ecx, dword ptr [esp + 12]
        ;; then these are moved back to the stack (esp + 16) 
        mov     byte ptr [esp + 16], al
        ;; the padding at esp +17/18/19 is uninitialized
        mov     dword ptr [esp + 20], ecx
        ;; and then moved back and forth a couple of times
        movsd   xmm0, qword ptr [esp + 16]
        movsd   qword ptr [esp], xmm0
        ;; and then we call foo, which never sees 42
        call    example::foo
        add     esp, 24
        ret
