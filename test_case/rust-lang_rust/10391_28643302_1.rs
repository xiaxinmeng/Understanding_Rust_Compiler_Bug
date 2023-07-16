
baz.rs:19:4: 19:21 error: internal compiler error: Cannot relate bound region: ReInfer(0) <= ReLateBound(34, BrNamed(syntax::ast::DefId{crate: 0, node: 52}, a))
This message reflects a bug in the Rust compiler.
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
baz.rs:19     Decodable::decode(d)
              ^~~~~~~~~~~~~~~~~
