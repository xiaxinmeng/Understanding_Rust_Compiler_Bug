rust
trait Trait {
    const VALUE: usize = 999;
}

struct Struct {}

impl Trait for Struct {}

impl Struct {
    const fn get_usize(&self) -> usize {
        1
    }
}

// Does not refer to `Trait` externally at all.
const fn get_usize(s: &Struct) -> usize {
    // Technically, either side of the addition operation below could become
    // invalid code if the person who maintains `Struct` changes it, assuming
    // that's not the same person who maintains this `get_usize` free function.
    s.get_usize() + <Struct as Trait>::VALUE
}

const STRUCT: Struct = Struct {};

const U: usize = get_usize(&STRUCT);

fn main() {
    println!("{}", U);
}
