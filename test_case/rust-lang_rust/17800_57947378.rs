
error: internal compiler error: ID not mapped to struct fields: variant Value in Value (id=5)
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:169

stack backtrace:
   1:        0x10f3ee269 - rt::backtrace::imp::write::he06af82d7d9ee38ecJq
   2:        0x10f3f15a1 - failure::on_fail::h9e3a39356479ba70QZq
   3:        0x10f6718e5 - unwind::begin_unwind_inner::h756181834df20b6dLud
   4:        0x10eb6db97 - unwind::begin_unwind::h18322048352354653950
   5:        0x10eb6e313 - diagnostic::Handler::bug::hdccf4ba5f46333fdISF
   6:        0x10c035ec8 - driver::session::Session::bug::hff02dab935ebf9e9Rtx
   7:        0x10c29179d - middle::ty::lookup_struct_fields::hdc6f7da2745a4112GEH
   8:        0x10c53839c - middle::typeck::check::_match::check_struct_like_enum_variant_pat::hb9adcf40078b738eX0J
   9:        0x10c52cbf1 - middle::typeck::check::_match::check_pat::h1cad16f9457d5170Y3J
  10:        0x10c52c47c - middle::typeck::check::_match::check_match::h640de9c08ab75b45ICJ
  11:        0x10c5c9931 - middle::typeck::check::check_expr_with_unifier::h6b59588f69798b15aNV
  12:        0x10c593745 - middle::typeck::check::check_block_with_expected::ha05c279d52ef53e60XX
  13:        0x10c58ecd2 - middle::typeck::check::check_fn::h0754b88f038f27adRsS
  14:        0x10c58e275 - middle::typeck::check::check_bare_fn::h6db665b3affe0759ZhS
  15:        0x10c58a0b8 - middle::typeck::check::check_item::h82af91f6696c38872JS
  16:        0x10c58e0df - middle::typeck::check::check_item_types::h55796e1a31aa1c3c9gS
  17:        0x10c04b4b6 - util::common::time::h10131786875316863222
  18:        0x10c83eb6e - middle::typeck::check_crate::h95db7480b8900497Edk
  19:        0x10c8a867f - driver::driver::phase_3_run_analysis_passes::h8decd58d8d4c066eEKw
  20:        0x10c8a3558 - driver::driver::compile_input::heb0d35abb1543609qrw
  21:        0x10c922584 - driver::run_compiler::h420de67eacbe487cHhA
  22:        0x10c920726 - driver::run::closure.146515
  23:        0x10c063bfb - task::TaskBuilder<S>::try_future::closure.101324
  24:        0x10c063af3 - task::TaskBuilder<S>::spawn_internal::closure.101295
  25:        0x10bfe36bd - task::spawn_opts::closure.8536
  26:        0x10f6d49ec - rust_try_inner
  27:        0x10f6d49d6 - rust_try
  28:        0x10f66f157 - unwind::try::h6df6339e56c18041tjd
  29:        0x10f66efec - task::Task::run::hf7474df0e760efdfEzc
  30:        0x10bfe3512 - task::spawn_opts::closure.8475
  31:        0x10f67083a - thread::thread_start::h960eee62f4c643ffPTc
  32:     0x7fff89670899 - _pthread_body
  33:     0x7fff8967072a - _pthread_struct_init
