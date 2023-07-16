 rust
#![crate_type="lib"]

extern {
    fn foo(x: i32);
}

pub mod x {
    #[no_mangle]
    pub unsafe extern "C" fn foo() { ::foo(0) }
}
