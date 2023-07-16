rust
#[repr(u32)]
pub enum Foo {
    A = 2,
}

pub enum Bar {
	A(Foo),
	B,
    C,
}

fn main() {
    match Bar::A(Foo::A) {
        Bar::A(_) => (),
        _ => unreachable!(),
    }
}
