rust
trait Foo {
    const FOO: impl Default;
}

impl Foo for X {
    const FOO: i32 = 42;
}
