
$ RUST_BACKTRACE=1 rustc ice.rs
ice.rs:9:9: 9:25 error: field `a` bound twice in pattern
ice.rs:9     let Foo {a: _, a: _} = bar;
                 ^~~~~~~~~~~~~~~~
error: internal compiler error: no type for node 28: pat _ (id=28) in fcx 0x7f5e08fefad8
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/nathan/devel/rust/rust/src/libsyntax/diagnostic.rs:163

stack backtrace:
   1:     0x7f5e0ae9bdf0 - rt::backtrace::imp::write::h108d0991ca026317UGp::v0.11.0.pre
   2:     0x7f5e0aea2920 - failure::on_fail::he2a6dc919a12a36691p::v0.11.0.pre
   3:     0x7f5e0cfe6ae0 - unwind::begin_unwind_inner::h08f08f512eb17f89cZd::v0.11.0.pre
   4:     0x7f5e0b9d1440 - unwind::begin_unwind::h13260005051934678122::v0.11.0.pre
   5:     0x7f5e0b9d1de0 - diagnostic::Handler::bug::h0882f89fc46fc1f1Pcc::v0.11.0.pre
   6:     0x7f5e0d95c040 - driver::session::Session::bug::he3dbad1ac7e3e51eG5q::v0.11.0.pre
   7:     0x7f5e0dbdff00 - middle::typeck::check::FnCtxt<'a>::node_ty::hf6396e49799c2931iwe::v0.11.0.pre
   8:     0x7f5e0dc0cb50 - middle::typeck::check::writeback::WritebackCx<'cx>::visit_node_id::h17bb9920cdaade20Zs7::v0.11.0.pre
   9:     0x7f5e0dc0c620 - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_pat::hdedc2230fb7b55231k7::v0.11.0.pre
  10:     0x7f5e0dc0c620 - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_pat::hdedc2230fb7b55231k7::v0.11.0.pre
  11:     0x7f5e0dc10360 - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_local::hf0efa79175ff7e1ceo7::v0.11.0.pre
  12:     0x7f5e0dc13e30 - visit::walk_block::h14268814427612348425::v0.11.0.pre
  13:     0x7f5e0dc10360 - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_local::hf0efa79175ff7e1ceo7::v0.11.0.pre
  14:     0x7f5e0dc13e30 - visit::walk_block::h14268814427612348425::v0.11.0.pre
  15:     0x7f5e0dc0c330 - middle::typeck::check::writeback::resolve_type_vars_in_fn::h514f032f67c0638fQc7::v0.11.0.pre
  16:     0x7f5e0dc6c4c0 - middle::typeck::check::check_bare_fn::hfca1b44af4ea2f6cRJc::v0.11.0.pre
  17:     0x7f5e0dc65f10 - middle::typeck::check::check_item::hb7411e26fcaa75d45gd::v0.11.0.pre
  18:     0x7f5e0dc6c2c0 - middle::typeck::check::check_item_types::hff3da1dc9ec71e4a9Ic::v0.11.0.pre
  19:     0x7f5e0dde9850 - middle::typeck::check_crate::h27a4add4caf7a098SJC::v0.11.0.pre
  20:     0x7f5e0e261c30 - driver::driver::phase_3_run_analysis_passes::hee64a98da1762f5a84p::v0.11.0.pre
  21:     0x7f5e0e25d1e0 - driver::driver::compile_input::h426af5bcce52e33f6Rp::v0.11.0.pre
  22:     0x7f5e0e329f90 - driver::run_compiler::h5bd01faab2919e705Bs::v0.11.0.pre
  23:     0x7f5e0e329ea0 - driver::main_args::closure.117253
  24:     0x7f5e0e33efc0 - task::TaskBuilder<S>::try_future::closure.118393
  25:     0x7f5e0e33ebe0 - task::TaskBuilder<S>::spawn_internal::closure.118370
  26:     0x7f5e0d344ca0 - task::spawn_opts::closure.7461
  27:     0x7f5e0d047140 - rust_try
  28:     0x7f5e0cfe30f0 - unwind::try::h442319c5f595eac5BNd::v0.11.0.pre
  29:     0x7f5e0cfe2dd0 - task::Task::run::h493de1a4392e540eXYc::v0.11.0.pre
  30:     0x7f5e0d344a60 - task::spawn_opts::closure.7407
  31:     0x7f5e0cfe5990 - thread::thread_start::h62060143786fc79bvld::v0.11.0.pre
  32:     0x7f5e0a913000 - start_thread
  33:     0x7f5e0ccaefc9 - __clone
  34:                0x0 - <unknown>
