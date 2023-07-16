rust
struct Article {
    proof_reader: ProofReader,
}

struct ProofReader {
    name: String,
}

pub trait HaveRelationship<To> {
    fn get_relation(self) -> To;
}

impl<'a> HaveRelationship<&'a ProofReader> for &'a Article {
    fn get_relation(self) -> &'a ProofReader {
        &self.proof_reader
    }
}

fn main() {
    println!("Hello, world!");
}
