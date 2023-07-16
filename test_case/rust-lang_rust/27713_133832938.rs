 rust
pub trait Hasher {
    ...
    /// Hash only the given bytes, and immediately finalize.
    /// Results are unspecified if other bytes were previously written to this hasher
    fn write_only(&mut self, bytes: &[u8]) -> u64 {
        self.write(bytes);
        self.finish()
    }
}

pub trait Hash {
    ...
    /// Hashes only this value
    fn hash_one_shot<H: Hasher>(&self, state: &mut H) -> u64 {
        self.hash(state);
        state.finish()
    }
}
