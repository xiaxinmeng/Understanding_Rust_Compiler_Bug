
struct Foo { ... }; // private

impl Foo {
    pub fn id(&self) -> int;
}

pub fn some_foo() -> Foo { ... }

// can a user write `some_foo().id()`, even though they cannot see `Foo`?
