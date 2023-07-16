rust
#[no_mangle] pub
unsafe
extern "C"
fn foo (_: *const Foo) {}

pub
struct Foo {
    _private: [u8; 0],
}
