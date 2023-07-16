 rust
struct NoCopy;

enum Test {
    MyVariant(NoCopy)
}

impl Copy for Test {}

fn main() {}
