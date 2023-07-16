
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 0 but the index is 0', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/librustc/lib.rs:1
stack backtrace:
   1:        0x10fcc9654 - rt::backtrace::imp::write::hf792c9a431d98150WEa::v0.11.pre
   2:        0x10fc2b58e - rt::unwind::begin_unwind_inner::hd86b006519df9a922ea::v0.11.pre
   3:        0x10fc2ad18 - rt::unwind::begin_unwind::h10174172393206223895::v0.11.pre
   4:        0x10fcc9119 - rt::unwind::begin_unwind_raw::hca7322628e40a5b5bca::v0.11.pre
   5:        0x10fc29f8e - rt::unwind::fail_::h7f000d2154e6cd34P99::v0.11.pre
   6:        0x10fcc9162 - rt::unwind::fail_bounds_check::closure.40249
   7:        0x10fc2c09e - rt::unwind::fail_bounds_check::h2ea0b1a4f61a3302caa::v0.11.pre
   8:        0x10d41d9e3 - middle::lint::Context<'a>.Visitor<(*>::visit_pat::h1dc0d8536dedb86a98B::v0.11.pre
   9:        0x10d423f4f - visit::walk_fn::h5356099215230029470::v0.11.pre
  10:        0x10d423ed0 - middle::lint::Context<'a>.Visitor<(*>::visit_fn::closure.70610
  11:        0x10d42435e - middle::lint::Context<'a>.Visitor<(*>::visit_fn::closure.70612
  12:        0x10d403f31 - middle::lint::Context<'a>::with_lint_attrs::hef3c7739e0013001mSA::v0.11.pre
  13:        0x10d41cac0 - middle::lint::Context<'a>.Visitor<(*>::visit_fn::h43414315bc8a5a061bC::v0.11.pre
  14:        0x10d415f1b - middle::lint::Context<'a>.Visitor<(*>::visit_item::closure.70596
  15:        0x10d403f31 - middle::lint::Context<'a>::with_lint_attrs::hef3c7739e0013001mSA::v0.11.pre
  16:        0x10d41cc76 - visit::Visitor::visit_mod::h12750473236215063983::v0.11.pre
  17:        0x10d42a8b7 - middle::lint::check_crate::closure.70667
  18:        0x10d403f31 - middle::lint::Context<'a>::with_lint_attrs::hef3c7739e0013001mSA::v0.11.pre
  19:        0x10d429f05 - middle::lint::check_crate::hdeb4e388e38f57d5jiC::v0.11.pre
  20:        0x10d3d680f - util::common::time::h6685874155479263413::v0.11.pre
  21:        0x10d7f19b1 - driver::driver::phase_3_run_analysis_passes::he6c312d7dc672e1cg1e::v0.11.pre
  22:        0x10d7f6db2 - driver::driver::compile_input::h5cc16b6b358a89603qf::v0.11.pre
  23:        0x10d81cba2 - run_compiler::hfaca67788353830ezUm::v0.11.pre
  24:        0x10d830a1d - main_args::closure.91224
  25:        0x10d82edb2 - monitor::closure.91099
  26:        0x10d82a8fb - task::TaskBuilder::try::closure.90865
  27:        0x10cfd848c - task::spawn_opts::closure.7106
  28:        0x10fcc4758 - rt::task::Task::run::closure.40146
  29:        0x10fcce4cc - rust_try
  30:        0x10fcc45d7 - rt::task::Task::run::h767d3727f94ffe61C57::v0.11.pre
  31:        0x10cfd830f - task::spawn_opts::closure.7078
  32:        0x10fcc8026 - rt::thread::thread_start::h6bde74cf5aa3b556PK8::v0.11.pre
  33:     0x7fff86d8b899 - _pthread_body
  34:     0x7fff86d8b72a - _pthread_struct_init

