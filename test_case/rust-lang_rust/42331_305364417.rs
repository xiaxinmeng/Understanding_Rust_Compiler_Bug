Rust
#![feature(attr_literals, repr_align)]

#[repr(align(256))]
struct Foo(usize);

fn main() {
    let mut foo = vec![Foo(273)];
    for i in 0..0x1000 {
        foo.reserve_exact(i);
        assert!(foo[0].0 == 273);
        assert!(foo.as_ptr() as usize & 0xff == 0);
        foo.shrink_to_fit();
        assert!(foo[0].0 == 273);
        assert!(foo.as_ptr() as usize & 0xff == 0);
    }
}
