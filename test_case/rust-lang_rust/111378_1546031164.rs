
error: local binding shadows glob re-export
   --> compiler/rustc_middle/src/ty/mod.rs:148:1
    |
73  | pub use rustc_type_ir::*;
    |         ---------------- the name `sty` in the type namespace is introduced by the glob reexport here
...
148 | mod sty;
    | ^^^^^^^^ but the local binding here shadows the name `sty` in the type namespace
    |
