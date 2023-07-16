
mod foo {  // private
    pub struct Foo { ... };

    impl Foo {
        pub fn id(&self) -> int;
    }
}

pub fn some_foo() -> foo::Foo { ... }

// can a user write `some_foo().id()`, even though they cannot see `foo::Foo` (because they cannot see `foo`)?
