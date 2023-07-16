rust
trait Foo<'a> {
    type In;
}

struct Simple<'a>(std::marker::PhantomData<&'a u32>);

fn somefn_simple(f: for<'a> fn(Simple<'a>) -> Simple<'a>) {
    // compiles.
}

fn somefn_gat<T: for<'r> Foo<'r>>(f: for<'a> fn(<T as Foo<'a>>::In) -> <T as Foo<'a>>::In) {
    // errors.
}

fn main() {}
