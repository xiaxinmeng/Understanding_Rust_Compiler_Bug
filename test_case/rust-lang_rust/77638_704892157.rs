
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen/mod.rs:121:9: Encountered errors `[FulfillmentError(Obligation(predicate=TraitPredicate(<dyn sc_client_api::PrunableStateChangesTrieStorage<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>, sp_runtime::OpaqueExtrinsic>> as sp_state_machine::changes_trie::Storage<sp_runtime::traits::BlakeTwo256, u32>>), depth=0),Unimplemented)]` resolving bounds after type-checking

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (98edd1fbf 2020-10-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

error: could not compile `node-testing`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
