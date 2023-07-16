rust
impl FnOnce<(u32,)> for Foo {
    type Output = u32;
    extern "rust-call" fn call_once(self, args: (u32,)) -> Self::Output {
        args.0
    }
}

impl FnOnce<(u32, u32)> for Foo {
    type Output = u32;
    extern "rust-call" fn call_once(self, args: (u32, u32)) -> Self::Output {
        args.0
    }
}
