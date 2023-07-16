plain
   Compiling structopt-derive v0.4.9
    Checking thiserror v1.0.20
    Checking structopt v0.3.16
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
error: unused import: `DUMMY_SP`
  |
  |
9 | use rustc_span::{symbol::kw, symbol::Ident, BytePos, Span, DUMMY_SP};
  |
note: the lint level is defined here
 --> src/tools/rustfmt/src/lib.rs:3:9
  |
  |
3 | #![deny(unused_imports)]

error: unused import: `AttrVec`
 --> src/tools/rustfmt/src/types.rs:4:28
  |
  |
4 | use rustc_ast::ast::{self, AttrVec, FnRetTy, Mutability};


error: unused import: `symbol::Ident`
  |
  |
5 | use rustc_span::{symbol::kw, symbol::Ident, BytePos, Pos, Span};

error: aborting due to 3 previous errors

error: could not compile `rustfmt-nightly`
