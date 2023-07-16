
10467.rs:5:4: 5:13 error: internal compiler error: Cannot relate bound region: ReLateBound(6, BrNamed(syntax::ast::DefId{crate: 0, node: 20}, self)) <= ReInfer(70)
This message reflects a bug in the Rust compiler. 
We would appreciate a bug report: https://github.com/mozilla/rust/wiki/HOWTO-submit-a-Rust-bug-report
10467.rs:5     cat_files(&args.iter().skip(1));
               ^~~~~~~~~
task 'rustc' failed at 'explicit failure', /home/huon/rust/src/libsyntax/diagnostic.rs:75
task '<main>' failed at 'explicit failure', /home/huon/rust/src/librustc/lib.rs:396
