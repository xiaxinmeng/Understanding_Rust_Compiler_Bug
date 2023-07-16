rust
pub trait HaveRelationship<To> {
    fn get_relation(&self) -> &To;
}

impl HaveRelationship<ProofReader> for Article {
    fn get_relation(&self) -> &ProofReader {
        &self.proof_reader
    }
}
