
cat src/lib.rs
pub trait Hasher {}

pub trait HashDB<H: Hasher>: AsHashDB<H> {}
pub trait AsHashDB<H: Hasher>: std::convert::AsRef<HashDB<H>> {}
