 rust
#![feature(core)]
struct Foo(u8);
impl Drop for Foo {
    fn drop(&mut self) { println!("Dropped Foo({})", self.0); self.0 += 1; }
}
struct Holder<T: ?Sized>(T);
fn main() {
    unsafe {
        let mut foo = Holder(Foo(0));
        &(*(&mut foo as &mut Holder<Send>)).0;
        &(*(&mut foo as &mut Holder<Send>)).0;
        &(*(&mut foo as &mut Holder<Send>)).0;
        std::mem::forget(foo);
    }
}
