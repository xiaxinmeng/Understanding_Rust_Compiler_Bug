rust
#![feature(associated_consts)]

trait Foo {
    const FOO: u32;
}

impl Foo for () {
    const FOO: u32 = 1;
}

fn main() {
    let _ = <Foo as Foo>::FOO;
}
