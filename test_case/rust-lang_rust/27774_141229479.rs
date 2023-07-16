
enum Foo {
    A, // no integers
    B(u32), // 1 integer
    C(Vec<u32>), // >1 integer
}

impl Foo {
    fn get_integers(&self) -> &[u32] {
        match *self {
            Foo::A => &[],
            Foo::B(ref i) => ref_slice(i),
            Foo::C(ref v) => v,
        }
    }
}
