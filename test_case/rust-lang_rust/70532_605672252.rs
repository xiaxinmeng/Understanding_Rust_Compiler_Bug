rust
trait Trait {
    type Assoc;
}
struct Foo<T: Trait>(T::Assoc);

impl Trait for fn(&'static ()) {
    type Assoc = u8;
}

impl Trait for for<'a> fn(&'a ()) {
    type Assoc = u16;
}

fn main() {
    use std::mem::size_of;
    
    dbg!(size_of::<Foo<fn(&'static ())>>());
    dbg!(size_of::<Foo<for<'b> fn(&'b ())>>());
}
