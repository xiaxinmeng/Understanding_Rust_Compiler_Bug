rust
trait Foo {
    type Error;
}

trait Bar {
    type Error;
}

trait Cake : Foo + Bar {}

trait Cake2 : Cake where Self : Foo<Error = i32>, Self : Bar<Error = u64> {}

struct S {
    f: Box<dyn Cake2>,
}
