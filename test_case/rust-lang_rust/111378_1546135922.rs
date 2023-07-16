
error: local binding shadows glob re-export
    --> compiler/rustc_span/src/source_map.rs:27:1
     |
  13 | pub use crate::*;
     |         -------- the name `tests` in the type namespace is introduced by the glob reexport here
  ...
  27 | mod tests;
     | ^^^^^^^^^^ but the local binding here shadows the name `tests` in the type namespace
     |
     = note: `-D local-binding-shadows-glob-reexport` implied by `-D warnings`
