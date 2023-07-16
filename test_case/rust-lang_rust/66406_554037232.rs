rust
pub trait HaveRelationship<'a, To: 'a> {
    fn get_relation(&'a self) -> To;
}

impl<'a> HaveRelationship<'a, &'a ProofReader> for Article {
    fn get_relation(&'a self) -> &'a ProofReader {
        &self.proof_reader
    }
}
