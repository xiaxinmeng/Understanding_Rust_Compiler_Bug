
task 'rustc' failed at '~Any', /private/tmp/rust-226T/src/libsyntax/diagnostic.rs:152
stack backtrace:
   1:        0x10bb71ef4 - rt::backtrace::imp::write::h9952562da44009cd8Vb::v0.11.pre
   2:        0x10bad6626 - rt::unwind::begin_unwind_inner::h818ce3415067025eCwb::v0.11.pre
   3:        0x10b214365 - rt::unwind::begin_unwind::hd3c81c1d9bdf98c6U51::v0.11.pre
   4:        0x10b214f5a - diagnostic::Handler::bug::h9808f62707358746AXb::v0.11.pre
   5:        0x109258398 - driver::session::Session::bug::hf9088fc0389765fd6bg::v0.11.pre
   6:        0x1092b8a49 - middle::ty::node_id_to_type::he2aebf80c5f8a445TQN::v0.11.pre
   7:        0x109a1929a - driver::driver::TypedAnnotation.pprust..PpAnn::post::h98c48e28661bf9dfhXe::v0.11.pre
   8:        0x10b2f9f76 - print::pprust::State$LT$$x27a$GT$::print_expr::h33585dacdf90326dCVN::v0.11.pre
   9:        0x10b2e6758 - print::pprust::State$LT$$x27a$GT$::print_type::hf7e626a4fe9e943asDM::v0.11.pre
  10:        0x10b2fef4a - print::pprust::State$LT$$x27a$GT$::print_item::h653328fb235e72c6ZRM::v0.11.pre
  11:        0x10b314b0b - print::pprust::State$LT$$x27a$GT$::print_decl::h3a13bbf50ec38568URO::v0.11.pre
  12:        0x10b2fb556 - print::pprust::State$LT$$x27a$GT$::print_stmt::haa621e58e9c9cf28WEN::v0.11.pre
  13:        0x10b31627c - print::pprust::State$LT$$x27a$GT$::print_block_maybe_unclosed::h3020700d3818b2f0JJN::v0.11.pre
  14:        0x10b2f725e - print::pprust::State$LT$$x27a$GT$::print_expr::h33585dacdf90326dCVN::v0.11.pre
  15:        0x10b2fba63 - print::pprust::State$LT$$x27a$GT$::print_stmt::haa621e58e9c9cf28WEN::v0.11.pre
  16:        0x10b31627c - print::pprust::State$LT$$x27a$GT$::print_block_maybe_unclosed::h3020700d3818b2f0JJN::v0.11.pre
  17:        0x10b2fdff9 - print::pprust::State$LT$$x27a$GT$::print_item::h653328fb235e72c6ZRM::v0.11.pre
  18:        0x10b2e32c8 - print::pprust::State$LT$$x27a$GT$::print_mod::h2939f4ea7b485139czM::v0.11.pre
  19:        0x10b2e268e - print::pprust::print_crate::hf0b4308afc2b5ccfP5L::v0.11.pre
  20:        0x109a19d7a - driver::driver::pretty_print_input::h3eac009c3eb959a32Ye::v0.11.pre
  21:        0x109a39ed4 - run_compiler::hb6e78f99a8ed4010Jim::v0.11.pre
  22:        0x109a4e30d - main_args::closure.91927
  23:        0x109a4cb22 - monitor::closure.91812
  24:        0x109a486cb - task::TaskBuilder::try::closure.91587
  25:        0x10920f83c - task::spawn_opts::closure.7928
  26:        0x10bb6d318 - rt::task::Task::run::closure.41812
  27:        0x10bb77c0c - rust_try
  28:        0x10bb6d197 - rt::task::Task::run::hf57d234e8d19fe63Fp9::v0.11.pre
  29:        0x10920f6bf - task::spawn_opts::closure.7900
  30:        0x10bb708d6 - rt::thread::thread_start::h15d71cb8d65d97ccm49::v0.11.pre
  31:     0x7fff98550899 - _pthread_body
  32:     0x7fff9855072a - _pthread_struct_init
