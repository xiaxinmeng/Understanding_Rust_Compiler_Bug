 rust
#![crate_type="lib"]

struct A(u32);

impl Drop for A {
    fn drop(&mut self) {
        self.0 = 0;
    }
}

fn foo() -> u32 {
    let a = &(A(1) as A);
    a.0
}
