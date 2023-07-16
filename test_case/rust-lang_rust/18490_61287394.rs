
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: *start <= *end', /build/rust-git/src/rust/src/libcore/slice.rs:396

stack backtrace:
   1:     0x7fc07401d550 - rt::backtrace::imp::write::h6d7b51dbe08c7a90Weq
   2:     0x7fc074020650 - <unknown>
   3:     0x7fc078371330 - unwind::begin_unwind_inner::ha369021e3864117bdSd
   4:     0x7fc078370e50 - unwind::begin_unwind_fmt::h6c1ae8b41766cb91FPd
   5:     0x7fc078370e10 - rust_begin_unwind
   6:     0x7fc0783b4d60 - panicking::panic_fmt::h1f4102d9dbcc2a5dh7j
   7:     0x7fc0783b0610 - panicking::panic::h68bf04de05fcd341l4j
   8:     0x7fc078de3300 - middle::typeck::check::vtable::check_object_safety::h5e858634fb68b05e6zN
   9:     0x7fc078de2c40 - middle::typeck::check::vtable::check_object_cast::h76abd899d5e17912cqN
  10:     0x7fc078e3ae20 - <unknown>
  11:     0x7fc078e617a0 - <unknown>
  12:     0x7fc078e5fb30 - <unknown>
  13:     0x7fc078e5e7a0 - <unknown>
  14:     0x7fc078e617a0 - <unknown>
  15:     0x7fc078eaeba0 - middle::typeck::check::check_stmt::hd0a4ef48c8f354dcjI0
  16:     0x7fc078e2f060 - <unknown>
  17:     0x7fc078e617a0 - <unknown>
  18:     0x7fc078ddbf10 - middle::typeck::check::_match::check_match::haa2decc95e3c7268D2M
  19:     0x7fc078e617a0 - <unknown>
  20:     0x7fc078e2f060 - <unknown>
  21:     0x7fc078e2b0a0 - <unknown>
  22:     0x7fc078e2ade0 - <unknown>
  23:     0x7fc078e31d00 - <unknown>
  24:     0x7fc078e270c0 - middle::typeck::check::check_item::h9547b3316e8784c0aBV
  25:     0x7fc078e290e0 - <unknown>
  26:     0x7fc078e2a8b0 - middle::typeck::check::check_item_types::h88799901145244c6bgV
  27:     0x7fc0789b2a50 - <unknown>
  28:     0x7fc0790bed90 - middle::typeck::check_crate::h62ec16fdebb5bcbeQZn
  29:     0x7fc079122330 - driver::driver::phase_3_run_analysis_passes::h9af115960c945947BJA
  30:     0x7fc07911d0a0 - driver::driver::compile_input::h7fc95caa4ba569b2mqA
  31:     0x7fc0791a0660 - <unknown>
  32:     0x7fc0791a0550 - <unknown>
  33:     0x7fc0789cb380 - <unknown>
  34:     0x7fc0789cb170 - <unknown>
  35:     0x7fc0786b3a80 - <unknown>
  36:     0x7fc0783c29f0 - <unknown>
  37:     0x7fc0783c29e0 - rust_try
  38:     0x7fc07836eae0 - unwind::try::hca7256b355f2fe07xGd
  39:     0x7fc07836e970 - task::Task::run::hf4a989e4dbd46d6cnMc
  40:     0x7fc0786b37c0 - <unknown>
  41:     0x7fc0783701d0 - <unknown>
  42:     0x7fc0733ce250 - start_thread
  43:     0x7fc07804b3b9 - clone
  44:                0x0 - <unknown>
