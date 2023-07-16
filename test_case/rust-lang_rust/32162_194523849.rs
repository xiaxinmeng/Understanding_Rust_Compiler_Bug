
$ RUST_BACKTRACE=1 rustc nan.rs
nan.rs:1:13: 1:27 warning: path statement with no effect, #[warn(path_statements)] on by default
nan.rs:1 fn main() { std::f64::NAN; }
                     ^~~~~~~~~~~~~~
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: type_is_zero_size(bcx.ccx(), bcx.tcx().node_id_to_type(expr.id))', ../src/librustc_trans/trans/expr.rs:160
stack backtrace:
   1:     0x7f4b45f41190 - sys::backtrace::tracing::imp::write::h3e252ef592ef6f37cHv
   2:     0x7f4b45f4a4db - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44999
   3:     0x7f4b45f4a033 - panicking::default_handler::h5a98c984225a759d1nA
   4:     0x7f4b45f0d62c - sys_common::unwind::begin_unwind_inner::ha2071187f9cf0076vvu
   5:     0x7f4b443b68af - sys_common::unwind::begin_unwind::h6879981865330546681
   6:     0x7f4b44431e5c - trans::expr::trans_into::h442d2e799eabdbf8vOD
   7:     0x7f4b4449f8bd - trans::controlflow::trans_stmt_semi::h628defac0bb772eensy
   8:     0x7f4b4442e7c0 - trans::controlflow::trans_block::h4e2329e57fef8e84jty
   9:     0x7f4b4442d2ce - trans::base::trans_closure::h048e0a4dbba11e08bPj
  10:     0x7f4b4442f4cc - trans::base::trans_fn::ha0a1212e94a7a2bfk0j
  11:     0x7f4b44433bd2 - trans::base::trans_item::hddf070bbf3233c95ook
  12:     0x7f4b4445130b - trans::base::TransItemsWithinModVisitor<'a, 'tcx>.Visitor<'v>::visit_item::h8c989fcd7781a6fbAol
  13:     0x7f4b4443ea8b - trans::base::trans_crate::h7e804094513fc4bf07k
  14:     0x7f4b46482434 - driver::phase_4_translate_to_llvm::h64dba564589e9b9eA1a
  15:     0x7f4b46480c7c - driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::closure.31105
  16:     0x7f4b4647d547 - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.30136
  17:     0x7f4b464772f4 - middle::ty::context::TyCtxt<'tcx>::create_and_enter::h10650487897641485352
  18:     0x7f4b46473dfb - driver::phase_3_run_analysis_passes::h16813158488996304421
  19:     0x7f4b46445ff5 - driver::compile_input::h9035c95dc59d6c56Kca
  20:     0x7f4b46433bdf - run_compiler::h6ec17a1dc91ec28c8Oc
  21:     0x7f4b46431301 - sys_common::unwind::try::try_fn::h6514687846834432941
  22:     0x7f4b45f3eb7b - __rust_try
  23:     0x7f4b45f36e2d - sys_common::unwind::inner_try::h4a24c2847e3d9a74xsu
  24:     0x7f4b46431b4a - boxed::F.FnBox<A>::call_box::h3348061696621889229
  25:     0x7f4b45f48aa9 - sys::thread::Thread::new::thread_start::h486bc7814d54d94ellz
  26:     0x7f4b3e691554 - start_thread
  27:     0x7f4b45bb7dec - __clone
  28:                0x0 - <unknown>
