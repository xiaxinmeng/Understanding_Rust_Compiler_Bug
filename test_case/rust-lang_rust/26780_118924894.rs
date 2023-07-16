 rust
#[derive(PartialEq, Eq, Hash)]
struct Atom { magic: u64 }
impl Deref for Atom { Target = str; ... }
