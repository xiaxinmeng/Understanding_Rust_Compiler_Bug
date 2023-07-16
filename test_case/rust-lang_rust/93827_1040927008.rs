
#![feature(const_fn_fn_ptr_basics)]

const fn foo() -> fn(&mut i32) {
    fn bar(_: &mut i32) {}

    bar
}
