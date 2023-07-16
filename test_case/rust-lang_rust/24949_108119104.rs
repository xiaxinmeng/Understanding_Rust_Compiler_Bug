
trait Foo {
    const FOO: Self;
}

impl Foo for u8 {
    const FOO: u8 = u8::FOO;
}
