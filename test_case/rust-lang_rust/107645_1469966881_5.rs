rust
fn foo() -> impl Debug { }

// becomes

type Foo = impl Debug;

#[defines(Foo)]
fn foo() -> Foo { ]
