rust
fn main() {
    struct Foo(u8, [u8]);
    struct Bar(String, [u64]);
    let ptr: *const Foo = unimplemented!();
    ptr as *const Bar;
}
