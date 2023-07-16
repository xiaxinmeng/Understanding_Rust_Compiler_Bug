 rust
struct S;

impl Iterator<uint> for S {
    fn next(&mut self) -> Result<uint, uint> { Ok(7) }
}

fn main() {}
