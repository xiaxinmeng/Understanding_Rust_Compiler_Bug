rust
trait Foo {
    fn get_x(&self) -> u32;
}

trait FooC {
    const X: u32;
}

impl FooC for () {
    const X: u32 = 3;
}

impl Foo for () {
    fn get_x(&self) -> u32 { <() as FooC>::X } ; // or just 3
}

