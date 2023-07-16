 Rust
// These are accepted
trait A: {}
fn b<'a:, U:>() {}
type C = for<'a> Clone+;

// These are forbidden

// bounds on where clauses must be non empty
fn d<T>() where T: {}

// In type grammar, `+` is treated like a binary operator,
// and hence both L and R side are required.
fn e(f: &(A+)) {}
