plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:15bd69b4
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:00:08]    Compiling libc v0.2.40
[01:00:08]    Compiling void v1.0.2
[01:00:08]    Compiling lazy_static v1.0.0
[01:00:08]    Compiling regex v0.2.10
[01:00:08] error: expected type, found `{`
[01:00:08]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.0.0/src/lazy.rs:16:23
[01:00:08]    |
[01:00:08] 16 | impl<T: Sync> Lazy<T> {
[01:00:08] 
[01:00:08] error: aborting due to previous error
[01:00:08] 
[01:00:08] 
[01:00:08] error: Could not compile `lazy_static`.
[01:00:08] warning: build failed, waiting for other jobs to finish...
[01:00:08] error: internal compiler error: librustc_typeck/collect.rs:1202: unexpected sort of node in fn_sig(): NodeItem(Item { name: main, id: NodeId(7), hir_id: HirId { owner: DefIndex(0:6), local_id: ItemLocalId(0) }, attrs: [], node: ItemFn(FnDecl { inputs: [], output: DefaultReturn(/cargo/registry/src/github.com-1ecc6299db9ec823/regex-0.2.10/build.rs:5:11: 5:11), variadic: false, has_implicit_self: false }, Normal, NotConst, Rust, Generics { params: [], where_clause: WhereClause { id: NodeId(8), predicates: [] }, span: /cargo/registry/src/github.com-1ecc6299db9ec823/regex-0.2.10/build.rs:1:1: 1:1 }, BodyId { node_id: NodeId(87) }), vis: Inherited, span: /cargo/registry/src/github.com-1ecc6299db9ec823/regex-0.2.10/build.rs:5:1: 27:2 })
[01:00:08] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[01:00:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:08] error: aborting due to previous error
[01:00:08] 
[01:00:08] 
[01:00:08] 
[01:00:08] note: the compiler unexpectedly panicked. this is a bug.
[01:00:08] 
[01:00:08] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:00:08] 
[01:00:08] note: rustc 1.27.0-nightly (d4c554bd0 2018-05-15) running on x86_64-unknown-linux-gnu
[01:00:08] 
[01:00:08] note: compiler flags: -C opt-level=3 -C linker=clang -C linker=clang --crate-type bin
[01:00:08] 
[01:00:08] note: some of the compiler flags provided by cargo are hidden
[01:00:08] error: Could not compile `regex`.
[01:00:08] warning: build failed, waiting for other jobs to finish...
[01:00:08] warning: build failed, waiting for other jobs to finish...
[01:00:08] error: Could not compile `void`.
[01:00:08] warning: build failed, waiting for other jobs to finish...
[01:00:08] error: Could not compile `libc`.
[01:00:08] To learn more, run the command again with --verbose.
[01:00:08] 
[01:00:08] 
[01:00:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml"
