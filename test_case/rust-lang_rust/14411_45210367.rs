
$ /opt/rust/bin/rustc --version
rustc 0.11.0-pre (0c74911 2014-06-05 00:51:48 -0700)
host: x86_64-unknown-linux-gnu
$ uname -a
Linux test 3.14-1-amd64 #1 SMP Debian 3.14.4-1 (2014-05-13) x86_64 GNU/Linux
$ RUST_BACKTRACE=1 /opt/rust/bin/rustc -g test.rs 
test.rs:1:1: 1:1 error: internal compiler error: debuginfo::set_members_of_composite_type() - Already completed forward declaration re-encountered.
test.rs:1 fn test(a: &Vec<u8>) {
          ^
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /tmp/rust/src/libsyntax/diagnostic.rs:106
stack backtrace:
   1:     0x7fb260921110 - rt::backtrace::imp::write::h5b011fbced4690bc0vx::v0.11.0.pre
   2:     0x7fb2608ab760 - rt::unwind::begin_unwind_inner::h4995bd0c39c49d8415w::v0.11.0.pre
   3:     0x7fb25f699340 - rt::unwind::begin_unwind::h18387254772016977913::v0.11.0.pre
   4:     0x7fb25f699270 - diagnostic::SpanHandler::span_bug::h1ebafc0cc5381e817Yb::v0.11.0.pre
   5:     0x7fb26140ca00 - driver::session::Session::span_bug::h3cb0e5e4a1a58907iAn::v0.11.0.pre
   6:     0x7fb261519830 - middle::trans::debuginfo::set_members_of_composite_type::h9d6b6d26b490c2fdSJB::v0.11.0.pre
   7:     0x7fb261518180 - middle::trans::debuginfo::RecursiveTypeDescription::finalize::hfc1025fa8d8596bezdB::v0.11.0.pre
   8:     0x7fb26150c430 - middle::trans::debuginfo::type_metadata::h26dd280d0653fbfa9bC::v0.11.0.pre
   9:     0x7fb26150c430 - middle::trans::debuginfo::type_metadata::h26dd280d0653fbfa9bC::v0.11.0.pre
  10:     0x7fb2615228d0 - middle::trans::debuginfo::vec_slice_metadata::h0ec8841b6b25512f20B::v0.11.0.pre
  11:     0x7fb26150c430 - middle::trans::debuginfo::type_metadata::h26dd280d0653fbfa9bC::v0.11.0.pre
  12:     0x7fb2615104c0 - middle::trans::debuginfo::declare_local::h9ad0c49244a464fcqVA::v0.11.0.pre
  13:     0x7fb26150f7a0 - middle::trans::debuginfo::create_local_var_metadata::closure.65228
  14:     0x7fb261908ce0 - middle::pat_util::pat_bindings::closure.77495
  15:     0x7fb25f6e3530 - ast_util::walk_pat::hfb30fd5660745952G8v::v0.11.0.pre
  16:     0x7fb2613e4c40 - middle::trans::controlflow::trans_stmt::h71e5e3cdf054599bX0a::v0.11.0.pre
  17:     0x7fb2613e6d20 - middle::trans::controlflow::trans_block::h781cb455d116b1e645a::v0.11.0.pre
  18:     0x7fb2614273f0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h159ddfe7ad51b9ebBaf::v0.11.0.pre
  19:     0x7fb2613e6610 - middle::trans::expr::trans_into::hcac93d6d00849afdJke::v0.11.0.pre
  20:     0x7fb2614dca80 - middle::trans::_match::trans_match_inner::h28ab07adb784f590Bbt::v0.11.0.pre
  21:     0x7fb2614273f0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h159ddfe7ad51b9ebBaf::v0.11.0.pre
  22:     0x7fb2613e6610 - middle::trans::expr::trans_into::hcac93d6d00849afdJke::v0.11.0.pre
  23:     0x7fb2613e57f0 - middle::trans::controlflow::trans_stmt_semi::h3f66501f21849971b5a::v0.11.0.pre
  24:     0x7fb2613e4c40 - middle::trans::controlflow::trans_stmt::h71e5e3cdf054599bX0a::v0.11.0.pre
  25:     0x7fb2613e6d20 - middle::trans::controlflow::trans_block::h781cb455d116b1e645a::v0.11.0.pre
  26:     0x7fb2614a8000 - middle::trans::base::trans_closure::h5f727b60571a9bceM7o::v0.11.0.pre
  27:     0x7fb2613af740 - middle::trans::base::trans_fn::h82bf58d3dcfe40ed9fp::v0.11.0.pre
  28:     0x7fb2613a8a40 - middle::trans::base::trans_item::hf8978f932565ffa2Xwp::v0.11.0.pre
  29:     0x7fb2614b81a0 - middle::trans::base::trans_crate::hc6f568a97ee25e04Ypq::v0.11.0.pre
  30:     0x7fb261c992b0 - driver::driver::phase_4_translate_to_llvm::h182ce6d3422188faZKm::v0.11.0.pre
  31:     0x7fb261c8e9c0 - driver::driver::compile_input::h4d9d1d436c03b1d3Asm::v0.11.0.pre
  32:     0x7fb261d59880 - driver::main_args::closure.94807
  33:     0x7fb261d6e5a0 - driver::monitor::closure.95878
  34:     0x7fb261d69ac0 - task::TaskBuilder::try::closure.95641
  35:     0x7fb260e50a90 - task::spawn_opts::closure.7844
  36:     0x7fb26091b110 - rt::task::Task::run::closure.25203
  37:     0x7fb2609803c0 - rust_try
  38:     0x7fb26091b060 - rt::task::Task::run::h1c418364160adcf9FUu::v0.11.0.pre
  39:     0x7fb260e50840 - task::spawn_opts::closure.7817
  40:     0x7fb26091f430 - rt::thread::thread_start::hbbafda47cc55f18ddEv::v0.11.0.pre
  41:     0x7fb25e3b8000 - start_thread
  42:     0x7fb260569fc9 - __clone
  43:                0x0 - <unknown>

$ 
