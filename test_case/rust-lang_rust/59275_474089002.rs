rust
struct Foo;
struct Bar;
struct Foobar;

trait Trait {
    type Foo;

    fn foo() -> Self::Foo;
    fn also_foo() -> Foo;
}

// current syntax:
impl Trait for Foobar {
    type Foo = Bar;

    fn foo() -> Self::Foo {
       Bar // everything nice and clear
    }
    fn also_foo() -> Foo {
        Foo
    }
}

// discussed syntax:
impl Trait for Foobar {
    type Foo = Bar;

    fn foo() -> Foo {
        Bar // ☝🏻 Foo is actually Bar 🤷🏻‍♂️
    }
    fn also_foo() -> Foo {
        Foo
    }
}
