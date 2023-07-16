 Rust
struct A<'a> {
    a: ::std::marker::PhantomData<&'a ()>
}

struct B {
    b: for<'a> A<'a>
}

fn main(){}
