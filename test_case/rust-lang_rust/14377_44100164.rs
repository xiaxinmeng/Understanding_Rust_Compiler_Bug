 rust
enum Enum<'a> {
    Empty,
    V(Vec<Enum<'a>>) // Removing this line, or changing Vec to Box, makes the ICE go away
}

impl<'a> Enum<'a> {
    fn consume(_: Enum<'a>) { }
}

fn main() { }
