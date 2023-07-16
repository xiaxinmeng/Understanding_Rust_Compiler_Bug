rust
// #[repr(C)]
union Foo {
    a: (),
    _b: u32,
}

fn main() {
    unsafe {
        let f: Foo = std::mem::MaybeUninit::uninit().assume_init();
        dbg!(f.a)
    }
}
