
rustc --out-dir build src/cat.rs
task 'rustc' failed at 'called `Option::unwrap()` on a `None` value', /private/tmp/rust-Q8uz/rust-0.10/src/libstd/option.rs:264
stack backtrace:
   1:        0x1084c68e4 - rt::backtrace::imp::write::h2a8a3b7117b6a3bag8b::v0.10
   2:        0x10842ab56 - rt::unwind::begin_unwind_inner::h13ee063055871ef1KIb::v0.10 
   3:        0x108429fc8 - rt::unwind::begin_unwind::hcf46aa8743077363kIb::v0.10
   4:        0x1084b1b11 - fmt::format_unsafe::hd5116fa5c4c9ce50cAS::v0.10
   5:        0x10845e4d1 - fmt::format::h8bf4d0c9e4e704d3UzS::v0.10
   6:        0x107ba5bc2 - diagnostic::print_diagnostic::hefa9ac952c0a6cb3u2b::v0.10
   7:        0x107ba3b1a - diagnostic::EmitterWriter.Emitter::emit::h06089cbc7baed50ax8b::v0.10
   8:        0x107ba18e4 - diagnostic::Handler::fatal::h345b5e2971fa50cbvSb::v0.10
   9:        0x105bd6118 - driver::session::Session::fatal::h8002452597140fadteh::v0.10
  10:        0x10624b573 - back::link::llvm_err::h6cd068d05a4f3b2ehl1::v0.10
  11:        0x10624b73f - back::link::WriteOutputFile::hd096c6e8f4b782f4dm1::v0.10
  12:        0x106250f3f - back::link::write::run_passes::closure.83784
  13:        0x105f3d13f - util::common::time::h9acad76c21817c9aXHh::v0.10
  14:        0x10624d1cd - back::link::write::run_passes::hbe6513f82bc31607so1::v0.10
  15:        0x106350aa1 - driver::driver::phase_5_run_llvm_passes::closure.90137
  16:        0x105f3d13f - util::common::time::h9acad76c21817c9aXHh::v0.10
  17:        0x10635093d - driver::driver::phase_5_run_llvm_passes::h22117e32ed7247datHf::v0.10
  18:        0x1063534b2 - driver::driver::compile_input::haa7c9f4729d6243csWf::v0.10
  19:        0x1063786c4 - run_compiler::hfd5ecc9785222053Yzn::v0.10
  20:        0x10638c36d - main_args::closure.92066
  21:        0x10638ab82 - monitor::closure.91951
  22:        0x10638672b - task::TaskBuilder::try::closure.91726
  23:        0x105b4649c - task::spawn_opts::closure.7957
  24:        0x1084c1d08 - rt::task::Task::run::closure.41637
  25:        0x1084cc5fc - rust_try
  26:        0x1084c1b87 - rt::task::Task::run::h544c274b867f177dNB9::v0.10
  27:        0x105b4631f - task::spawn_opts::closure.7929
  28:        0x1084c52c6 - rt::thread::thread_start::hc06c53d926f6e6a3uga::v0.10
  29:     0x7fff8edac899 - _pthread_body
  30:     0x7fff8edac72a - _pthread_struct_init
