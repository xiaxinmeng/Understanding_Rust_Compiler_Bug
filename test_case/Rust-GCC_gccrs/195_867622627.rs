rust
fn foo() {
    let a = 0;
    let b = || a;
    b();
}

// -->

fn foo() {
    struct MyClosure<'a> {
        a: &i32,
    }
    impl<'a> Fn<()> for MyClosure<'a> {
        type Output = i32;
        extern "rust-call" fn call(&self, args: ()) -> i32 {
            *self.a
        }
    }
    let a = 0;
    let b = MyClosure { a: &a };
    b();
}
