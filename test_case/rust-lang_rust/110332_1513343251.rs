rs
fn f<T: Trait>() {
    let _: T::Item /* Res::Err */ = loop {};
}

fn main() {
    f::<Struct>();
}

struct Struct;
trait Trait { type Item; }
impl Trait for Struct { type Item = (); }
