rust
trait Foo {
    const FOO: impl Default;
}

impl Foo for X {
    const FOO: impl Default = 42;
}
