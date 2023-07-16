asm
example::panics:
        push    rax
        call    std::panicking::begin_panic
        ud2

.L__unnamed_3:
        .quad   core::ptr::drop_in_place<&str>
        .asciz  "\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
        .quad   <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::take_box
        .quad   <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get

.L__unnamed_4:
        .quad   core::ptr::drop_in_place<&str>
        .asciz  "\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
        .quad   <T as core::any::Any>::type_id

.L__unnamed_1:
        .ascii  "a panic"

.L__unnamed_5:
        .ascii  "/app/example.rs"

.L__unnamed_2:
        .quad   .L__unnamed_5
        .asciz  "\017\000\000\000\000\000\000\000\002\000\000\000\005\000\000"
