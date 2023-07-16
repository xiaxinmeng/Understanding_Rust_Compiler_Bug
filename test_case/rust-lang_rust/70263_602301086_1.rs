rust
struct Manual;
impl<'a> FnOnce<(&'a i32,)> for Manual {
    type Output = &'a i32;
    extern "rust-call" fn call_once(self, (x,): (&'a i32,)) -> &'a i32 { x }
}
foo(Manual); // OK
