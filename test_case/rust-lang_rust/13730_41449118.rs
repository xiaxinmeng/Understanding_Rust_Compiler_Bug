
shell $> RUST_BACKTRACE=1 rustc test.rs
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'No triple in crate', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libstd/option.rs:245
stack backtrace:
   1:        0x102e24844 - rt::backtrace::imp::write::h254d0a6dd1fae24biCa::v0.11.pre
   2:        0x102d868de - rt::unwind::begin_unwind_inner::h6e474448729455a4oca::v0.11.pre
   3:        0x1000f5d58 - rt::unwind::begin_unwind::h11500859575803473188::v0.11.pre
   4:        0x100845b87 - metadata::decoder::get_crate_triple::h55d8343241d15157rQb::v0.11.pre
   5:        0x10085ed0a - metadata::loader::Context<'a>::extract_one::hed57ece8b2ef4096nbe::v0.11.pre
   6:        0x10085836a - metadata::loader::Context<'a>::find_library_crate::h1f72c6a1abaf2bedGEd::v0.11.pre
   7:        0x100853e0f - metadata::loader::Context<'a>::load_library_crate::h5e7923cd92f974214qd::v0.11.pre
   8:        0x10084fd64 - metadata::creader::resolve_crate::h611b40168b787d362rc::v0.11.pre
   9:        0x100848d7b - metadata::creader::Env<'a>.visit..Visitor<(*>::visit_view_item::h65cca42091d33f2fj1b::v0.11.pre
  10:        0x1008480bd - metadata::creader::read_crates::h7532ce1f7c6177deq0b::v0.11.pre
  11:        0x1004c279f - util::common::time::h14699901310049870540::v0.11.pre
  12:        0x1008dcd6d - driver::driver::phase_3_run_analysis_passes::h8e28097da6e59f487Df::v0.11.pre
  13:        0x1008e3f62 - driver::driver::compile_input::h5c1c854541bd3a7eU3f::v0.11.pre
  14:        0x100909dd2 - run_compiler::h62e9fe55eaa01b0dLAn::v0.11.pre
  15:        0x10091dc4d - main_args::closure.91516
  16:        0x10091bfe2 - monitor::closure.91391
  17:        0x100917b2b - task::TaskBuilder::try::closure.91157
  18:        0x102a7347c - task::spawn_opts::closure.7105
  19:        0x102e1f948 - rt::task::Task::run::closure.40126
  20:        0x102e296bc - rust_try
  21:        0x102e1f7c7 - rt::task::Task::run::h67dcf35100df5a07Y27::v0.11.pre
  22:        0x102a732ff - task::spawn_opts::closure.7077
  23:        0x102e23216 - rt::thread::thread_start::he31c712e1f859f82bI8::v0.11.pre
  24:     0x7fff96a42899 - _pthread_body
  25:     0x7fff96a4272a - _pthread_struct_init
