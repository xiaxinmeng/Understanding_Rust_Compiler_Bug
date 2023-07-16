
#![crate_type = "lib"]

extern {
    fn printf(s: *const i8, ...);
}

pub unsafe fn foo() {
    printf("foo\n\0".as_ptr() as *const _);
}
