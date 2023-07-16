
thread 'rustc' panicked at 'internal error: entered unreachable code', /Users/shep/Projects/rust/src/librustc/middle/ty.rs:910

stack backtrace:
   1:        0x10aaf9868 - sys::backtrace::write::h9a580fabc37cb8adktt
   2:        0x10ab1a3c3 - failure::on_fail::h468f41665a3fd1e84Iz
   3:        0x10aa85aaa - rt::unwind::begin_unwind_inner::h11db2db05da0fb99vqz
   4:        0x107dc853c - rt::unwind::begin_unwind::h13685703016598346556
   5:        0x1081b9e09 - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::confirm_candidate::h2e81d0fae36ca86ar9Z
   6:        0x1081ca050 - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::winnow_candidate::unboxed_closure.88573
   7:        0x1081bcaae - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::winnow_candidate::hce2b651d441f597djHZ
   8:        0x1081c4987 - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::candidate_from_obligation_no_cache::hcd23f9522bf5cd3aJaZ
   9:        0x1081b49f0 - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::candidate_from_obligation::h604b896a505825bei2Y
  10:        0x1081b11f3 - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::select::h983db20fb6e85e80oHY
  11:        0x1081aae0b - middle::traits::fulfill::FulfillmentContext<$u{27}tcx$GT$::select::hb8d69b6e6df55003o6X
  12:        0x108029c58 - middle::traits::fulfill::FulfillmentContext<$u{27}tcx$GT$::select_all_or_error::h8797a47d7fcfcb8e92X
  13:        0x107546ba6 - check::vtable::select_all_fcx_obligations_or_error::h589d8c7447f73e49zfb
  14:        0x10760dab2 - check::check_bare_fn::hccca5d8239b94ddfPck
  15:        0x107605ae3 - check::check_item::h4573d3f167765844fwk
  16:        0x107845530 - check_crate::unboxed_closure.43034
  17:        0x107840cdd - check_crate::h0e6b79165f7057d1GZy
  18:        0x10735cb6e - driver::phase_3_run_analysis_passes::h3f041cda7348a5efEta
  19:        0x10733fbae - driver::compile_input::h94fd7f9d3c75da33rba
  20:        0x1074b71d5 - run_compiler::hac67bdfebe9cef0bAYb
  21:        0x1074b49bf - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h13992406362119359971
  22:        0x1074b2f59 - rt::unwind::try::try_fn::h6067637045518264477
  23:        0x10ab83149 - rust_try_inner
  24:        0x10ab83136 - rust_try
  25:        0x1074b3657 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h13712900278951656686
  26:        0x10ab09884 - sys::thread::thread_start::h8539aa31b7c6ba86Pow
  27:     0x7fff93e6e2fc - _pthread_body
  28:     0x7fff93e6e279 - _pthread_body
