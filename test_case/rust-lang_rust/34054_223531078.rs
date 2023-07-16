 rust
#![crate_type="lib"]
struct S {
    x: i8,
    y: i8,
}

extern {
    fn foo(s: S);
}

pub fn bla() {
    unsafe {
        foo(S { x: 4, y: 8 });
    }
}
