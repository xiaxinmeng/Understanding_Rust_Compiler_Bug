\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[],"rendered":"error[E0601]: main function not found\n\n"}
[01:00:25] thread 'rustc' panicked at 'non-FnLike node found: NodeTraitItem(TraitItem { id: NodeId(6), name: f, hir_id: HirId { owner: DefIndex(0:4), local_id: ItemLocalId(0) }, attrs: [], generics: Generics { params: [], where_clause: WhereClause { id: NodeId(7), predicates: [] }, span: /checkout/src/test/ui/issue-47706-trait.rs:1:1: 1:1 }, node: Method(MethodSig { unsafety: Normal, constness: NotConst, abi: Rust, decl: FnDecl { inputs: [type(&Self), type(())], output: DefaultReturn(/checkout/src/test/ui/issue-47706-trait.rs:12:24: 12:24), variadic: false, has_implicit_self: true } }, Provided(BodyId { node_id: NodeId(22) })), span: /checkout/src/test/ui/issue-47706-trait.rs:12:5: 14:6 })', librustc/traits/error_reporting.rs:870:18
[01:00:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:25] 
[01:00:25] error: internal compiler error: unexpected panic
[01:00:25] 
[01:00:25] note: the compiler unexpectedly panicked. this is a bug.
[01:00:25] 
[01:00:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:00:25] 
[01:00:25] note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu
[01:00:25] 
[01:00:25] 
[01:00:25] ------------------------------------------
[01:00:25] 
[01:00:25] thread '[ui] ui/issue-47706-trait.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:00:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:25] 
[01:00:25] 
[01:00:25] failures:
[01:00:25]     [ui] ui/issue-47706-trait.rs
[01:00:25] 
[01:00:25] test result: FAILED. 985 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
