rust
struct Container<'a>(std::marker::PhantomData<&'a ()>);

struct Empty;

trait Trait {
    type Assoc;
}

impl<'a> Trait for Container<'a> {
    type Assoc = Empty;
}

fn foo(x: impl for <'a> FnOnce(<Container<'a> as Trait>::Assoc)) {
    x(Empty);
}
