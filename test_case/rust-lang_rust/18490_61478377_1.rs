
task 'rustc' panicked at 'assertion failed: *start <= *end', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libcore/slice.rs:396

stack backtrace:
   1:        0x10d65d619 - rt::backtrace::imp::write::h03fee16d4a0b6b41moq
   2:        0x10d66085c - failure::on_fail::h3e0f6b3c6c4bbcc3VEq
   3:        0x10d8c4f05 - unwind::begin_unwind_inner::h6c7f4532e72658d7SJd
   4:        0x10d8c4b9f - unwind::begin_unwind_fmt::h9863b25c438d6314kHd
   5:        0x10d8c48f2 - rust_begin_unwind
   6:        0x10d91b53c - panicking::panic_fmt::hc53a414d3d53805bi7j
   7:        0x10d911f6f - panicking::panic::h5cf45d77d6cb56acm4j
   8:        0x10a84c83b - middle::typeck::check::vtable::check_object_safety::h3ee32f70bc2a6910cvN
   9:        0x10a84bab4 - middle::typeck::check::vtable::check_object_cast::ha17cfa6c915c29failN
  10:        0x10a8b4954 - middle::typeck::check::check_cast::h212d4526506e607bdwW
  11:        0x10a8dcae4 - middle::typeck::check::check_expr_with_unifier::h153e2ccf9b0cb061XCY
  12:        0x10a92c583 - middle::typeck::check::check_decl_local::h4f8f86e796500762GD0
  13:        0x10a92c6f7 - middle::typeck::check::check_stmt::h7691c17f92d0623eGF0
  14:        0x10a8a7faf - middle::typeck::check::check_block_with_expected::heb9a0fb2300c4559PJ0
  15:        0x10a8a3d42 - middle::typeck::check::check_fn::h0449d63e5b4f3e3a9oV
  16:        0x10a8a326f - middle::typeck::check::check_bare_fn::hf2de12c52654551eoeV
  17:        0x10a89f18d - middle::typeck::check::check_item::hfaeea23703d7dd28xyV
  18:        0x10a8a30bf - middle::typeck::check::check_item_types::haaff85b74e988d92ydV
  19:        0x10a3b9556 - util::common::time::h11920059333862041717
  20:        0x10ab6359e - middle::typeck::check_crate::h50fcc5c8b39993f4LVn
  21:        0x10abcbbcf - driver::driver::phase_3_run_analysis_passes::h1425a4ca465c7b097DA
  22:        0x10abc62b8 - driver::driver::compile_input::h60bc42ef7fabbfd4SkA
  23:        0x10ac43ca8 - driver::run_compiler::h4031243f49082a9fw7D
  24:        0x10ac41ece - driver::run::closure.145349
  25:        0x10a3d181b - task::TaskBuilder<S>::try_future::closure.103709
  26:        0x10a3d1713 - task::TaskBuilder<S>::spawn_internal::closure.103680
  27:        0x10a35198d - task::NativeSpawner.Spawner::spawn::closure.8526
  28:        0x10d92b20c - rust_try_inner
  29:        0x10d92b1f6 - rust_try
  30:        0x10d8c2677 - unwind::try::hab84862d0081b274cyd
  31:        0x10d8c250c - task::Task::run::h39093ecd59554b49ZJc
  32:        0x10a3517b3 - task::NativeSpawner.Spawner::spawn::closure.8463
  33:        0x10d8c3d37 - thread::thread_start::h589be8c0a398d299e5c
  34:     0x7fff8bbe7899 - _pthread_body
  35:     0x7fff8bbe772a - _pthread_struct_init
