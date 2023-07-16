rust
extern {
    static SOME_STATIC: &u32;
}

fn foo() -> Option<u32> {
    if call_c_init() {
         // okay, C initialized it
         *SOME_STATIC
    } else { None }
}
