asm
example::example:
; example() arguments:
; rdi = data ptr to `A<dyn Any>`
; rsi = vtable ptr to an instantiation of `Any`

        mov     rax, qword ptr [rsi + 16] ; load alignment of DST field (`alignof(T)`) from vtable

        add     rax, 3  ; these two instructions effectively compute
        and     rax, -4 ; `max(sizeof(i32), alignof(T))`, relying on both being powers of 2

        add     rdi, rax ; offset data pointer from start of `A` to `A.b`

        jmp     qword ptr [rsi + 24] ; call `Any::type_id`
