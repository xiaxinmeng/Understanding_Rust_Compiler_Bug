
mbp:cargo achin$ env RUST_BACKTRACE=1 DYLD_LIBRARY_PATH=${PWD}/rustc/lib/ PATH=${PWD}/rustc/bin/:${PATH} rustc --crate-type lib a.rs -l static=z -L native=/opt/local/lib 
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Custom(Custom { kind: Other, error: StringError("failed to open archive") }) }', ../src/libcore/result.rs:732

stack backtrace:
   1:        0x104193bc0 - sys::backtrace::write::ha29ed9eeda09b191Yts
   2:        0x10419be01 - panicking::on_panic::h4f0311883f3e75d9Z7w
   3:        0x10415f512 - rt::unwind::begin_unwind_inner::hcd1c9ab2da35c9410Cw
   4:        0x104160107 - rt::unwind::begin_unwind_fmt::hfb293892a08f91906Bw
   5:        0x10419b9f7 - rust_begin_unwind
   6:        0x1041e76a0 - panicking::panic_fmt::h687aa220bbe8cf9dqgC
   7:        0x1007fa2a1 - back::link::link_rlib::h928bc1e051a71c40nQb
   8:        0x1007f72c3 - back::link::link_binary::h3416077127894b5fRCb
   9:        0x10067b533 - driver::phase_6_link_output::haa465d1c10610748kRa
  10:        0x100650ac0 - driver::compile_input::h71e7015e5f60fc20Tba
  11:        0x10072e39e - run_compiler::he0b8e9edf87ff758C7b
  12:        0x10072bf8b - boxed::F.FnBox<A>::call_box::h710433560290561307
  13:        0x10072b8e2 - rt::unwind::try::try_fn::h4210567753699827351
  14:        0x10419b9a8 - __rust_try
  15:        0x104187e70 - rt::unwind::try::inner_try::h332543bb689c0967Tyw
  16:        0x10072baa2 - boxed::F.FnBox<A>::call_box::h13997262008008063974
  17:        0x10419ac5d - sys::thread::Thread::new::thread_start::h6180cb2a7d6f1601eXv
  18:     0x7fff84ada898 - _pthread_body
  19:     0x7fff84ada729 - _pthread_start
