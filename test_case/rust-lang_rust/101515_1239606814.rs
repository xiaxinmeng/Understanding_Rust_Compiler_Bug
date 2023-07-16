rust
/// Useful type to use with `Result<>` indicate that an error has already
/// been reported to the user, so no need to continue checking.
#[derive(Clone, Copy, Debug, Encodable, Decodable, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(HashStable_Generic)]
pub struct ErrorGuaranteed(());
