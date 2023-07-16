
~/Projekte$ RUST_BACKTRACE=1 rustc rust-bug.rs                                                              101 ↵ 
rust-bug.rs:10:9: 10:18 error: internal compiler error: Explicit index of non-index type `It`
rust-bug.rs:10     for [x, y, z] in It {
                       ^~~~~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:113

stack backtrace:
   1:     0x7fc16f9208d0 - rt::backtrace::imp::write::h9a97349a494f89bePUq
   2:     0x7fc16f923a90 - failure::on_fail::hb11547504289a1ffhgr
   3:     0x7fc1700f22d0 - unwind::begin_unwind_inner::h5df2dc6f14b19f71MTd
   4:     0x7fc16e3b8620 - unwind::begin_unwind::h1365354703164647196
   5:     0x7fc16e3b85a0 - diagnostic::SpanHandler::span_bug::h810a8fac103d8c1cMhF
   6:     0x7fc170697460 - driver::session::Session::span_bug::h976022b1a0303ee7m8w
   7:     0x7fc170a13d20 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_index::h6849561118867022794
   8:     0x7fc170a12100 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern::h3339052091073497715
   9:     0x7fc1709fa8e0 - middle::typeck::check::regionck::visit_expr::hd3c8b825c2773d621PM
  10:     0x7fc1709f7a80 - middle::typeck::check::regionck::Rcx<'a, 'tcx>::visit_fn_body::hc777c8ab8612eedbyqM
  11:     0x7fc1709f7990 - middle::typeck::check::regionck::regionck_fn::h83e3b6c2d3d53709tjM
  12:     0x7fc170a3a130 - middle::typeck::check::check_bare_fn::hfebc4617d948ec12E8R
  13:     0x7fc170a35ec0 - middle::typeck::check::check_item::h77a7f21c8a144855HAS
  14:     0x7fc170a39be0 - middle::typeck::check::check_item_types::h697321cef90212deO7R
  15:     0x7fc1704e85d0 - util::common::time::h18398786739644959194
  16:     0x7fc170ce1010 - middle::typeck::check_crate::h63bb01e16b1996cd4Uj
  17:     0x7fc170d4ab40 - driver::driver::phase_3_run_analysis_passes::h57b76357b4c15009Opw
  18:     0x7fc170d46290 - driver::driver::compile_input::hc03497de7e6c909eL6v
  19:     0x7fc170dc8360 - driver::run_compiler::h24a8daf18bbc3b70DWz
  20:     0x7fc170dc8240 - driver::main_args::closure.146005
  21:     0x7fc170501710 - task::TaskBuilder<S>::try_future::closure.101291
  22:     0x7fc170501500 - task::TaskBuilder<S>::spawn_internal::closure.101262
  23:     0x7fc1716d8650 - task::spawn_opts::closure.8491
  24:     0x7fc170148480 - rust_try_inner
  25:     0x7fc170148470 - rust_try
  26:     0x7fc1700ef910 - unwind::try::h8c8d3e6eb0ca16a6uId
  27:     0x7fc1700ef770 - task::Task::run::hfb873359c59e1424cYc
  28:     0x7fc1716d83c0 - task::spawn_opts::closure.8431
  29:     0x7fc1700f1360 - thread::thread_start::h2b0150e66287e12frid
  30:     0x7fc16f3e7e70 - start_thread
  31:     0x7fc16fdbac09 - clone
  32:                0x0 - <unknown>
