
rustc 1.19.0-nightly (6a5fc9eec 2017-05-02)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:329
stack backtrace:
<snip>
   9: core::panicking::panic
             at /checkout/src/libcore/panicking.rs:49
  10: rustc_trans::cabi_x86_64::compute_abi_info::{{closure}}
  11: rustc_trans::abi::FnType::adjust_for_abi
  12: rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_block
  13: rustc_trans::mir::trans_mir
  14: rustc_trans::trans_item::TransItem::define
  15: rustc_trans::base::trans_crate
  16: rustc_driver::driver::phase_4_translate_to_llvm
<snip>
