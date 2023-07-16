rust
trait Foo {
    const FOO: impl Default = 1;
}

impl Foo for X {
    const FOO: impl Default = 42;
}
