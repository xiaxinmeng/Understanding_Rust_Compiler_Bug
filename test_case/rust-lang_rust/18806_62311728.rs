 rust
struct Foo<Sized? T> {
    foo: T
}

fn main() {
    let foo_sized: Foo<[uint, .. 4]> = Foo { foo: [1u,2,3,4] };
    let foo_unsized: &Foo<[uint]> = &foo_sized;
    for &i in foo_unsized.foo.iter() {
        println!("{}", i);
    }
}
