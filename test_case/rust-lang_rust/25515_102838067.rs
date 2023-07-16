 rust
#![feature(core)]
struct Foo(u8);
impl Drop for Foo {
    fn drop(&mut self) { println!("Dropped Foo({})", self.0); self.0 += 1; }
}
struct Holder<T: ?Sized>(T);
fn main() {
    unsafe {
        let ptr = std::mem::transmute::<_, *mut Holder<[Foo; 1]>>(Box::new(Holder([Foo(0)])));
        let big_ptr: *mut Holder<[Foo]> = ptr;
        std::intrinsics::drop_in_place(&mut (*big_ptr).0 as *mut _);
        // leak box memory here :(
    }
}
