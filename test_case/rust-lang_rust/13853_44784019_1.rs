
$ RUST_BACKTRACE=1 rustc testcase.rs
error: internal compiler error: ID not mapped to struct fields: enum NodeContents::NodeContents (id=3)
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:162
stack backtrace:
   1:     0x7ffee7a64c10 - rt::backtrace::imp::write::h4eaa694fcd948704Wrx::v0.11.0.pre
   2:     0x7ffee79e7730 - rt::unwind::begin_unwind_inner::h3b5fcee9a7d12dbfX1w::v0.11.0.pre
   3:     0x7ffee63be2f0 - rt::unwind::begin_unwind::h6952148588903257095::v0.11.0.pre
   4:     0x7ffee63bed30 - diagnostic::Handler::bug::h4b77aa25d705a445P4b::v0.11.0.pre
   5:     0x7ffee850fc70 - driver::session::Session::bug::hfc07277a564612feWGn::v0.11.0.pre
   6:     0x7ffee86200c0 - middle::ty::lookup_struct_fields::hfb681bcf73bb30d2TpQ::v0.11.0.pre
   7:     0x7ffee8561670 - middle::ty::struct_fields::he157a00b7c3060eb1uQ::v0.11.0.pre
   8:     0x7ffee86b1880 - middle::ty::type_contents::tc_ty::hcf3453b2500819dfPCN::v0.11.0.pre
   9:     0x7ffee865b040 - middle::ty::type_contents::h59dcd90524a31605LBN::v0.11.0.pre
  10:     0x7ffee8a28ac0 - middle::kind::check_item::h098689564b21e13a08S::v0.11.0.pre
  11:     0x7ffee8a2b7b0 - middle::kind::check_crate::h9e02f0748a8c41d7dZS::v0.11.0.pre
  12:     0x7ffee8dbcc60 - driver::driver::phase_3_run_analysis_passes::hc9c5bc13cc92c0a5kJm::v0.11.0.pre
  13:     0x7ffee8db6120 - driver::driver::compile_input::he7dad0e3dbb6fee3Uym::v0.11.0.pre
  14:     0x7ffee8e82540 - driver::run_compiler::h96a720220bae0c8cDdp::v0.11.0.pre
  15:     0x7ffee8e82460 - driver::main_args::closure.94809
  16:     0x7ffee8e97600 - driver::monitor::closure.95880
  17:     0x7ffee8e929e0 - task::TaskBuilder::try::closure.95643
  18:     0x7ffee7f97690 - task::spawn_opts::closure.7832
  19:     0x7ffee7a5f0a0 - rt::task::Task::run::closure.25246
  20:     0x7ffee7ac8ea0 - rust_try
  21:     0x7ffee7a5eff0 - rt::task::Task::run::h11c4d2099f444a65AQu::v0.11.0.pre
  22:     0x7ffee7f97440 - task::spawn_opts::closure.7805
  23:     0x7ffee7a63040 - rt::thread::thread_start::h43799e40ebb51dd18zv::v0.11.0.pre
  24:     0x7ffee54f40c0 - start_thread
  25:     0x7ffee76a72d9 - __clone
  26:                0x0 - <unknown>
