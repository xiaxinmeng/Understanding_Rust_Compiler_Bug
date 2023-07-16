rust
struct Article {
    proof_reader: ProofReader,
}

struct ProofReader {
    name: String,
}

pub trait HaveRelationship<'a, To> {
    fn get_relation(&'a self) -> To;
}

impl<'a> HaveRelationship<'a, &'a ProofReader> for Article {
    fn get_relation(&'a self) -> &'a ProofReader {
        &self.proof_reader
    }
}

fn main() {
    println!("Hello, world!");
}
