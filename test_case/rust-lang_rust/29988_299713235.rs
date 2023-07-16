rust
#[repr(C)]
struct S {
    f1: i32,
    f2: i32,
    f3: i32,
}

extern {
    fn foo(s: S);
}

fn main() {
    let s = S { f1: 1, f2: 2, f3: 3 };
    unsafe {
        foo(s);
    }
}
