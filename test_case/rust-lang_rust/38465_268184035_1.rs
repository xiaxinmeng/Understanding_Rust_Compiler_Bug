rust
#[naked]
unsafe extern "C" fn tim1() {
    asm! {
        "
        push	r15
        push	r14
        push	r13
        push	r12
        "
    }
    not_inlined_foo();
    asm! {
        "
        pop	r12
        pop	r13
        pop	r14
        pop	r15
        reti
        "
    }
}
