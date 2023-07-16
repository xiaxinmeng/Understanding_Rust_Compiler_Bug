rust
#[repr(align(16))]
pub struct Foo<T> {
    bar: T,
    baz: T,
}

#[inline(never)]
pub fn test(x: Foo<(i64, i64)>) -> Foo<(i64, i64)> {
    let bar = x.bar;
    let baz = x.baz;
    Foo { bar: (bar.1, baz.0), baz: (0, 0) }
}

fn main() {
    test(Foo { bar: (0, 0), baz: (0, 0) });
}
