
❯ cargo run
   Compiling town-blacksmith v0.1.0 (/home/user/dev/project)
thread 'rustc' panicked at 'found unstable fingerprints for predicates_of(core[3998]::ops::function::Fn): GenericPredicates { parent: None, predicates: [(Binder(TraitPredicate(<Self as std::ops::FnMut<Args>>)), /home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:67:21: 67:32 (#0)), (Binder(TraitPredicate(<Args as std::marker::Sized>)), /home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:67:14: 67:18 (#0)), (Binder(TraitPredicate(<Self as std::ops::Fn<Args>>)), /home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:67:1: 67:32 (#0))] }', /rustc/07e0e2ec268c140e607e1ac7f49f145612d0f597/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (07e0e2ec2 2021-03-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z share-generics=y -C embed-bitcode=no -C debuginfo=2 -C linker=/usr/bin/clang -C incremental -C link-arg=-fuse-ld=lld --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [predicates_of] computing predicates of `std::ops::Fn`
#1 [vtable_methods] finding all methods for trait std::ops::Fn
end of query stack
error: could not compile `town-blacksmith`

To learn more, run the command again with --verbose.
