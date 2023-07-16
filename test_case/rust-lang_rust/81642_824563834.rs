rust
#[repr(u32)]
enum Foo {
    Aa = 1,
    Bb = 3,
    Cc = 100,
}

impl From<Foo> for u32 {
    fn from(x: Foo) -> u32 { x as _ }
}

fn main() {
    dbg!(Foo::Aa.into());
}
