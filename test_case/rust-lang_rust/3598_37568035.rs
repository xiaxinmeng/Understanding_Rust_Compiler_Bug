 rust
#[lang="covariant_type"]
#[deriving(Eq,Clone)]  // error: cannot determine a type for this expression: unconstrained type
pub struct CovariantType<T>;
