rust
trait Trait {
    type Associated;
}

struct Type<'a, 'b, T> {
    a: &'a T,
    b: &'b &'a (),
}

impl<'a, 'b, T> Type<'a, 'b, T>
where
    T: Trait + 'a,
{
    fn foo(param: T::Associated) {
        Self::bar(param);
    }

    fn bar(_: T::Associated) {}
}
