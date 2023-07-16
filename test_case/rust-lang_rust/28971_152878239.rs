
thread 'rustc' panicked at 'path not fully resolved: PathResolution { base_def: DefTy(DefId { krate: 0, node: DefIndex(846) => bootstrap::Binding }, true), last_private: LastMod(AllPublic), depth: 1 }', ../src/librustc/middle/def.rs:83
stack backtrace:
   1:     0x7fd7fdccfdf0 - sys::backtrace::tracing::imp::write::h6835252f71ef0b43Ljt
   2:     0x7fd7fdcd66c5 - panicking::log_panic::_<closure>::closure.39533
   3:     0x7fd7fdcd6135 - panicking::log_panic::h9ae49eb755ee9e97Tjx
   4:     0x7fd7fdc99fb3 - sys_common::unwind::begin_unwind_inner::hd874242a6b5e38b5Pcs
   5:     0x7fd7fdc9a918 - sys_common::unwind::begin_unwind_fmt::ha26b26e4a1594894Vbs
   6:     0x7fd7fb9cabbb - middle::mem_categorization::_<impl>::cat_pattern_::h3836257055123789321
   7:     0x7fd7fb9ca142 - middle::expr_use_visitor::_<impl>::determine_pat_move_mode::he2eb6d0f3e644737N4e
   8:     0x7fd7fb9c6d00 - middle::expr_use_visitor::_<impl>::walk_expr::h514de3050c7b7d553ie
   9:     0x7fd7fb9c57d1 - middle::expr_use_visitor::_<impl>::consume_expr::h772bb1bce5ad860eKae
  10:     0x7fd7fb9c57d1 - middle::expr_use_visitor::_<impl>::consume_expr::h772bb1bce5ad860eKae
  11:     0x7fd7fb9c5456 - middle::expr_use_visitor::_<impl>::walk_fn::h52cc8d343d4e5a9dH5d
  12:     0x7fd7fc8a48ea - check::upvar::_<impl>::visit_fn::hde4ca4b137400eafoTj
  13:     0x7fd7fc8a44dc - visit::walk_expr::walk_expr::h787507935738902711
  14:     0x7fd7fc8a422a - visit::walk_expr::walk_expr::h787507935738902711
  15:     0x7fd7fc8a422a - visit::walk_expr::walk_expr::h787507935738902711
  16:     0x7fd7fc8a450a - visit::walk_expr::walk_expr::h787507935738902711
  17:     0x7fd7fc8ea285 - check::check_bare_fn::h0dcafc15a1172281xbp
  18:     0x7fd7fc8e7775 - check::check_item_body::h011d65348be548c4ABp
  19:     0x7fd7fc8e7c83 - visit::walk_item::walk_item::h12204036668747400340
  20:     0x7fd7fc99b07d - check_crate::ha5f8c59164e1f737ZpD
  21:     0x7fd7fe1a1779 - driver::phase_3_run_analysis_passes::_<closure>::closure.21914
  22:     0x7fd7fe18855c - middle::ty::context::_<impl>::create_and_enter::create_and_enter::h15773383436087710206
  23:     0x7fd7fe183f9d - driver::phase_3_run_analysis_passes::h12004779431882585896
  24:     0x7fd7fe164c8c - driver::compile_input::h448def6b1ea47a237ba
  25:     0x7fd7fe2b3c6b - run_compiler::h78af33ac2a1574900pc
  26:     0x7fd7fe2b0ce6 - sys_common::unwind::try::try_fn::try_fn::h14585449656343585994
  27:     0x7fd7fdccdb18 - __rust_try
  28:     0x7fd7fdcc1b6b - sys_common::unwind::try::inner_try::h50ec61b8f468aff5n9r
  29:     0x7fd7fe2b1034 - boxed::_<impl>::call_box::call_box::h13330059328811376786
  30:     0x7fd7fdcd5193 - sys::thread::_<impl>::new::thread_start::hadf418adebb492757Bw
  31:     0x7fd7f737a181 - start_thread
  32:     0x7fd7fd95147c - __clone
  33:                0x0 - <unknown>
