 rust
struct IndexableThing;

impl Index<uint, String> for IndexableThing {
    fn index(&self, rhs: &uint) -> &String {
        unimplemented!();
    }
}

impl Index<uint, Vec<uint>> for IndexableThing {
    fn index(&self, rhs: &uint) -> &Vec<uint> {
        unimplemented!();
    }
}

fn main() {}
