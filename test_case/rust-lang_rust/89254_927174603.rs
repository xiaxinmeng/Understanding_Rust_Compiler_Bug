rust
struct Shark;

impl PartialEq<Shark> for u8 {
    fn eq(&self, other: &Shark) -> bool {
        false
    }
}

fn main() {
    Vec::<u8>::new() == [];
}
