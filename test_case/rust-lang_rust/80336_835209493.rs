

   Compiling substrate-test-runtime v2.0.0 (/home/thales/substrate/test-utils/runtime)
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(f4ab1f83ea5f30bc-7172c6b02018f3b6): Ok(EvaluatedToOk)', /rustc/88f19c6dab716c6281af7602e30f413e809c5974/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0 (88f19c6da 2021-05-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `pallet_transaction_payment::ChargeTransactionPayment<Runtime>: REWARD_CURVE::_sp_runtime::traits::SignedExtension`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [REWARD_CURVE::_sp_runtime::generic::Block<REWARD_CURVE::_sp_runtime::generic::Header<u32, REWARD_CURVE::_sp_runtime::traits::BlakeTwo256>, REWARD_CURVE::_sp_runtime::generic::UncheckedExtrinsic<REWARD_CURVE::_sp_runtime::MultiAddress<REWARD_CURVE::_sp_runtime::AccountId32, u32>, Call, REWARD_CURVE::_sp_runtime::MultiSignature, (frame_system::CheckSpecVersion<Runtime>, frame_system::CheckTxVersion<Runtime>, frame_system::CheckGenesis<Runtime>, frame_system::CheckMortality<Runtime>, frame_system::CheckNonce<Runtime>, frame_system::CheckWeight<Runtime>, pallet_transaction_payment::ChargeTransactionPayment<Runtime>)>>], item_def_id: DefId(203:1528 ~ sp_runtime[653f]::traits::Block::Extrinsic) } } }`
end of query stack
error: could not compile `node-runtime`
