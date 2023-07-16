
$ RUST_BACKTRACE=1 cargo test
   Compiling calcr v0.0.1 (file:/home/patrick/Dropbox/projects/calcr)
<std macros>:8:12: 107:46 error: cannot apply unary operator `!` to type `core::result::Result<(),&'static str>`
<std macros>:8         if !$cond {
<std macros>:9             fail!($($arg),+)
<std macros>:10         }
<std macros>:11     );
<std macros>:12 )
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 12 but the index is 12', /home/rustbuild/src/rust-    buildbot/slave/nightly-linux/build/src/libsyntax/lib.rs:1

 stack backtrace:
   1: 0xb3e452c0 - rt::backtrace::imp::write::ha1c450b4915eb143gcq
   2: 0xb3e48310 - failure::on_fail::h9ff118121a195e26rxq
   3: 0xb64c9ef0 - unwind::begin_unwind_inner::h3a14f8fa4197b75aB8d
   4: 0xb64c9bc0 - unwind::begin_unwind_fmt::h2a42796a19e436e865d
   5: 0xb64c9b60 - rust_begin_unwind
   6: 0xb6513330 - failure::begin_unwind::h71b3c90806f8db38c0j
   7: 0xb6517420 - failure::fail_bounds_check::hf6a2343a99d2624fpYj
   8: 0xb5c94fb0 - codemap::FileMap::get_line::hbaafa5c76d06d454NiC
   9: 0xb5c9f5e0 - diagnostic::emit::h1207110484c28756WHD
  10: 0xb5c9c410 - diagnostic::EmitterWriter.Emitter::emit::h861c31368d14acf9xDD
  11: 0xb5c9a850 - diagnostic::Handler::emit::h3542b2b846a8b13dklD
  12: 0xb5c937d0 - diagnostic::SpanHandler::span_err::hcdfb6f94e1cfbe0eV9C
  13: 0xb6c02d20 -  middle::typeck::infer::InferCtxt<'a>::type_error_message_str_with_expected::h110dc04b6ee80582L8a
  14: 0xb6cb1b20 - middle::typeck::infer::InferCtxt<'a>::type_error_message::hbd6bb41bf684d94ebgb
  15: 0xb6cc6300 - middle::typeck::check::check_expr_with_unifier::check_user_unop::closure.121224
  16: 0xb6cc3e40 - middle::typeck::check::check_expr_with_unifier::lookup_op_method::h5ba548b428edda80MkT
  17: 0xb6cc5f00 - middle::typeck::check::check_expr_with_unifier::check_user_unop::hc81e6082c69f0039DzT
  18: 0xb6cb4990 - middle::typeck::check::check_expr_with_unifier::he69a3f50b075640416S
  19: 0xb6cb4990 - middle::typeck::check::check_expr_with_unifier::he69a3f50b075640416S
  20: 0xb6d17f90 - middle::typeck::check::check_stmt::hfe0c02bad49756f2MaV
  21: 0xb6c7ee70 - middle::typeck::check::check_block_with_expected::heb620c3431a8c912XeV
  22: 0xb6c7ae70 - middle::typeck::check::check_fn::hac63a018f2d6076fqpQ
  23: 0xb6c7ab90 - middle::typeck::check::check_bare_fn::hcec58f2bb6d2b9eegeQ
  24: 0xb6c74360 - middle::typeck::check::check_item::hefceed674604ae234MQ
  25: 0xb6c75f30 - visit::walk_item::h8881549312631385631
  26: 0xb6c75f30 - visit::walk_item::h8881549312631385631
  27: 0xb6c7a990 - middle::typeck::check::check_item_types::h7fc9f56244aa6515ydQ
  28: 0xb6e90990 - middle::typeck::check_crate::closure.126402
  29: 0xb66cbe00 - util::common::time::h5960456725657221026
  30: 0xb6e8f540 - middle::typeck::check_crate::hf8afaae1cb1a8e05Juh
  31: 0xb6f5eeb0 - driver::driver::phase_3_run_analysis_passes::hc41db9e0e1edb3ebLWv
  32: 0xb6f5a1c0 - driver::driver::compile_input::hf7082c9a93eed0120Iv
  33: 0xb6ff8b30 - driver::run_compiler::h95110376e75ff237INy
  34: 0xb6ff8a30 - driver::main_args::closure.134677
  35: 0xb700a1b0 - task::TaskBuilder<S>::try_future::closure.135824
  36: 0xb7009fa0 - task::TaskBuilder<S>::spawn_internal::closure.135801
  37: 0xb66134a0 - task::spawn_opts::closure.8299
  38: 0xb64c9960 - unwind::try::try_fn::h5b6a70a8d3115d40nZd
  39: 0xb6526f90 - rust_try
  40: 0xb64c75d0 - unwind::try::hfdf393c0a0edfa750Wd
  41: 0xb64c7390 - task::Task::run::ha7e1b260e3ee789eC4c
  42: 0xb6613270 - task::spawn_opts::closure.8242
  43: 0xb64c90a0 - thread::thread_start::he601f6c86aee42ccMsd
  44:        0x0 - <unknown>

Could not compile `calcr`.
