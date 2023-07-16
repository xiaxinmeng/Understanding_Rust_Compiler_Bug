
test3.rs:6:4: 6:20 error: internal compiler error: Cannot relate bound region: ReLateBound(18, BrNamed(syntax::ast::DefId{crate: 0, node: 31}, a)) <= ReLateBound(16, BrNamed(syntax::ast::DefId{crate: 0, node: 17}, a))
This message reflects a bug in the Rust compiler. 
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
test3.rs:6     FromVec::fromVec(v)
               ^~~~~~~~~~~~~~~~
