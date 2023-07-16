
#![feature(fn_traits)]
#![feature(unboxed_closures)]
pub struct A {}
impl std::ops::FnMut<()> for A {
    extern "rust-call" fn call_mut(&mut self, args: ()) { self.call_once(args) }
}
impl std::ops::FnOnce<()> for A {
    type Output = ();
    extern "rust-call" fn call_once(self, _args: ()) { }
}
