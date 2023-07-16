rust
trait SomeOtherTrait<'a> {
    fn create(string: &'a mut str) -> Self;
}

trait SomeTrait<'a> {
    type Associated;

    fn do_stuff(&self, other: &mut Self::Associated);
}

fn something_else<T>(v: T)
where
    for<'b> T: SomeTrait<'b>,
    for<'b> <T as SomeTrait<'b>>::Associated: SomeOtherTrait<'b>,
{
    let mut message = "Hello".to_string();

    {
        let evil_borrow = <T::Associated as SomeOtherTrait<'_>>::create(message.as_mut_str());
    }
}

struct FooAssociated<'a> {
    blah: &'a str,
}
impl<'a> SomeOtherTrait<'a> for FooAssociated<'a> {
    fn create(string: &'a mut str) -> Self {
        Self { blah: string }
    }
}

struct Foo {}
impl<'a> SomeTrait<'a> for Foo {
    type Associated = FooAssociated<'a>;

    fn do_stuff(&self, other: &mut Self::Associated) {
        // ...
    }
}

fn foo(x: Foo) {
    something_else(x);
}
