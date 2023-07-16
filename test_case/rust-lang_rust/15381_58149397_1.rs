 terminal
slice_match.rs:4:9: 4:16 error: internal compiler error: Explicit index of non-index type `core::iter::Filter<'_,&[u8],core::slice::Chunks<'_,u8>>`
slice_match.rs:4     for [x,y,z] in values.as_slice().chunks(3).filter(|&xs| xs.len() == 3) {
                         ^~~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:113

stack backtrace:
   1:        0x111c73269 - rt::backtrace::imp::write::he06af82d7d9ee38ecJq
   2:        0x111c765a1 - failure::on_fail::h9e3a39356479ba70QZq
   3:        0x111ef48e5 - unwind::begin_unwind_inner::h756181834df20b6dLud
   4:        0x1113e3b97 - unwind::begin_unwind::h18322048352354653950
   5:        0x1113e3b30 - unwind::begin_unwind::h18322048352354653950
   6:        0x10ea73b6d - driver::session::Session::span_bug::h2c0f6016b5fa651cwtx
   7:        0x10ede89fa - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_index::h13791422843569837526
   8:        0x10ede728d - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern::h13679513847658926920
   9:        0x10edd10e8 - middle::typeck::check::regionck::visit_expr::h82e747716217ebf1GUM
  10:        0x10edcc884 - middle::typeck::check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h3ac342a98dca80f6dvM
  11:        0x10edcbccd - middle::typeck::check::regionck::regionck_fn::h34dea5a76c44f2e38nM
  12:        0x10ee0f292 - middle::typeck::check::check_bare_fn::h6db665b3affe0759ZhS
  13:        0x10ee0b0b8 - middle::typeck::check::check_item::h82af91f6696c38872JS
  14:        0x10ee0f0df - middle::typeck::check::check_item_types::h55796e1a31aa1c3c9gS
  15:        0x10e8cc4b6 - util::common::time::h10131786875316863222
  16:        0x10f0bfb6e - middle::typeck::check_crate::h95db7480b8900497Edk
  17:        0x10f12967f - driver::driver::phase_3_run_analysis_passes::h8decd58d8d4c066eEKw
  18:        0x10f124558 - driver::driver::compile_input::heb0d35abb1543609qrw
  19:        0x10f1a3584 - driver::run_compiler::h420de67eacbe487cHhA
  20:        0x10f1a1726 - driver::run::closure.146515
  21:        0x10e8e4bfb - task::TaskBuilder<S>::try_future::closure.101324
  22:        0x10e8e4af3 - task::TaskBuilder<S>::spawn_internal::closure.101295
  23:        0x10e8616bd - task::spawn_opts::closure.8536
  24:        0x111f579ec - rust_try_inner
  25:        0x111f579d6 - rust_try
  26:        0x111ef2157 - unwind::try::h6df6339e56c18041tjd
  27:        0x111ef1fec - task::Task::run::hf7474df0e760efdfEzc
  28:        0x10e861512 - task::spawn_opts::closure.8475
  29:        0x111ef383a - thread::thread_start::h960eee62f4c643ffPTc
  30:     0x7fff8e93a899 - _pthread_body
  31:     0x7fff8e93a72a - _pthread_struct_init
