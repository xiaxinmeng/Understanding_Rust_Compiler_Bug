
macro_rules! foo {
  ($e:path) => { use crate::{$e, bar, foobar!()}; }
}
