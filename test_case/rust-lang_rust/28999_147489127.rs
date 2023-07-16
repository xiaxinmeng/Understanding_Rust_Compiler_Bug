 Rust
pub struct Xyz<'a, V> {
    v: (V, &'a u32),
}

fn eq<'a, 's, 't, V>(this: &'s Xyz<'a, V>, other: &'t Xyz<'a, V>) -> bool
        where V: PartialEq {
    this.v == other.v
}

fn main() {}
