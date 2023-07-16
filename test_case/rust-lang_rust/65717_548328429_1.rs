rust
#[assert_unwind_safe]
#[should_panic] #[test] fn foo() { unsafe {
    { let mut v = V.lock().unwrap(); v.reserve(42); v.set_len(42); }
    panic!();
}}
