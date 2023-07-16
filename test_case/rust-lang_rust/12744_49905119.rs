 rust
trait Foo {}
impl Foo for u8 {}

// Coercions work in argument position:

fn take_box(foo: Box<Foo>) {}
fn take_ref(foo: &Foo) {}

fn use_takes() {
    let x: u8 = 0;
    take_box(box x);
    take_ref(&x);
}

// But do not work in return position:

fn make_box() -> Box<Foo> {
    // The following fails with:
    //   error: mismatched types: expected `Box<Foo>` but found `Box<u8>`
    //   (expected trait Foo but found u8)
    // box 0u8

    box 0u8 as Box<Foo>
}

fn make_ref(u: &u8) -> &Foo {
    // The following fails with:
    //   error: mismatched types: expected `&Foo` but found `&u8`
    //   (expected trait Foo but found u8)
    // u

    u as &Foo
}

// They do work via ascription:

fn main() {
    let b: Box<Foo> = box 0u8;
    let r: &Foo = &0u8;
}
