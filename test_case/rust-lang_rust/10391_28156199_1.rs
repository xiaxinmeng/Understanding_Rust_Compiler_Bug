
foo.rs:5:4: 5:7 error: internal compiler error: Cannot relate bound region: ReLateBound(6, BrNamed(syntax::ast::DefId{crate: 0, node: 19}, a)) <= ReInfer(11)
This message reflects a bug in the Rust compiler.
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
foo.rs:5     foo(x.iter());
             ^~~
