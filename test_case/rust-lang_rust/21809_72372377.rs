 rust
// from
let v = (a..b).map(f).collect::<Vec<_>>();
// to
use std::iter::IntoIterator;
let v = (a..b).into_iter().map(f).collect::<Vec<_>>();
