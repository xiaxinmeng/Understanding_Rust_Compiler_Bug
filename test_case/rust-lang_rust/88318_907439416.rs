
error: internal compiler error: encountered incremental compilation error with predicates_of(server[2099]::DatabaseStore)
  |
  = help: This is a known issue with the compiler. Run `cargo clean -p influxdb_iox` or `cargo clean` to allow your project to compile
  = note: Please follow the instructions below to create a bug report with the provided information
  = note: See <https://github.com/rust-lang/rust/issues/84970> for more information

thread 'rustc' panicked at 'Found unstable fingerprints for predicates_of(server[2099]::DatabaseStore): GenericPredicates { parent: None, predicates: [(Binder(TraitPredicate(<Self as std::fmt::Debug>), []), /Users/carolnichols/rust/delorean/server/src/lib.rs:222:26: 222:41 (#0)), (Binder(TraitPredicate(<Self as std::marker::Send>), []), /Users/carolnichols/rust/delorean/server/src/lib.rs:222:44: 222:48 (#0)), (Binder(TraitPredicate(<Self as std::marker::Sync>), []), /Users/carolnichols/rust/delorean/server/src/lib.rs:222:51: 222:55 (#0)), (Binder(TraitPredicate(<Self as server::DatabaseStore>), []), /Users/carolnichols/rust/delorean/server/src/lib.rs:222:1: 239:2 (#0))] }', /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/compiler/rustc_query_system/src/query/plumbing.rs:620:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0 (a178d0322 2021-07-26) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [predicates_of] computing predicates of `server::DatabaseStore`
#1 [typeck] type-checking `influxdb_ioxd::rpc::management::<impl at src/influxdb_ioxd/rpc/management.rs:55:1: 550:2>::list_databases`
end of query stack
