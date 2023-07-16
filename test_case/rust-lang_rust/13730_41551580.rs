
rustc 0.11-pre (7a19a82 2014-04-27 23:06:41 -0700)
host: x86_64-apple-darwin

error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'No triple in crate', /Users/tony/Projects/rust/src/libstd/option.rs:245
stack backtrace:
   1:        0x10579ab54 - rt::backtrace::imp::write::h5284f6f000f6ae7bWEa::v0.11.pre
   2:        0x1056fca6e - rt::unwind::begin_unwind_inner::hc5e220aa52d760b32ea::v0.11.pre
   3:        0x102adad88 - rt::unwind::begin_unwind::h6977348054022664160::v0.11.pre
   4:        0x103228217 - metadata::decoder::get_crate_triple::hae33c85ffcba99a6dPb::v0.11.pre
   5:        0x10324126a - metadata::loader::Context<'a>::extract_one::hbae40514abfb3cb199d::v0.11.pre
   6:        0x10323a90e - metadata::loader::Context<'a>::find_library_crate::h1d5c45db7d0b039esDd::v0.11.pre
   7:        0x10323639f - metadata::loader::Context<'a>::load_library_crate::h92462d1eea86e631Qpd::v0.11.pre
   8:        0x1032322f4 - metadata::creader::resolve_crate::hb4385703dd61ea90Oqc::v0.11.pre
   9:        0x10322b40b - metadata::creader::Env<'a>.visit..Visitor<(*>::visit_view_item::hc9a02c7fe0c35bff5Zb::v0.11.pre
  10:        0x10322a74d - metadata::creader::read_crates::hed35785f57159931cZb::v0.11.pre
  11:        0x102ea73ff - util::common::time::h11798269188829317611::v0.11.pre
  12:        0x1032beead - driver::driver::phase_3_run_analysis_passes::h2d57781201399e76TCf::v0.11.pre
  13:        0x1032c60a2 - driver::driver::compile_input::h180868b11ee2bee3G2f::v0.11.pre
  14:        0x1032ebec2 - run_compiler::hea20b259c2288aeaxzn::v0.11.pre
  15:        0x1032ffd3d - main_args::closure.91451
  16:        0x1032fe0d2 - monitor::closure.91326
  17:        0x1032f9c1b - task::TaskBuilder::try::closure.91092
  18:        0x1053da77c - task::spawn_opts::closure.7106
  19:        0x105795c58 - rt::task::Task::run::closure.40156
  20:        0x10579f9fc - rust_try
  21:        0x105795ad7 - rt::task::Task::run::h4f0d7c454c92123fC57::v0.11.pre
  22:        0x1053da5ff - task::spawn_opts::closure.7078
  23:        0x105799526 - rt::thread::thread_start::h8daeca6a5e3b3ec6PK8::v0.11.pre
  24:     0x7fff8ae67899 - _pthread_body
  25:     0x7fff8ae6772a - _pthread_struct_init
