
foo.rs:6:4: 6:5 error: internal compiler error: Cannot relate bound region: ReLateBound(11, BrNamed(syntax::ast::DefId{crate: 0u32, node: 20u32}, a)) <= ReInfer(1)
This message reflects a bug in the Rust compiler.
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
foo.rs:6     c.my_iter();
             ^
