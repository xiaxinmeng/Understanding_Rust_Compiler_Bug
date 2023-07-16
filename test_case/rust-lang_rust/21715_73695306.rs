
$ touch empty.rs && RUST_BACKTRACE=1 rustc empty.rs -Z unstable-options
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'No option 'show-span' defined', /home/munksgaard/src/rust/src/libgetopts/lib.rs:293

stack backtrace:
   1:     0x7f3057a41f00 - sys::backtrace::write::hde4dc71c4c33b7c9L1y
   2:     0x7f3057a66300 - failure::on_fail::h25c49b66647ae6b0pcH
   3:     0x7f30579c42d0 - rt::unwind::begin_unwind_inner::h47f408c6e14ad979KQG
   4:     0x7f30579c4de0 - rt::unwind::begin_unwind_fmt::hee1681655e9b6171gPG
   5:     0x7f30522b7230 - Matches::opt_vals::ha882112fd1714097F5a
   6:     0x7f30522b8500 - Matches::opt_str::h4b4e5e14857cec0agbb
   7:     0x7f305810cd90 - run_compiler::h95361ebe24858b6b6bc
   8:     0x7f305810b3f0 - thunk::F.Invoke<A, R>::invoke::h7400904607630647539
   9:     0x7f305810a320 - rt::unwind::try::try_fn::h530882388650529344
  10:     0x7f3057add0d0 - rust_try_inner
  11:     0x7f3057add0c0 - rust_try
  12:     0x7f305810a5d0 - thunk::F.Invoke<A, R>::invoke::h18348562218329660567
  13:     0x7f3057a527f0 - sys::thread::thread_start::h6593a04931c6d32foQC
  14:     0x7f3051bb9140 - <unknown>
  15:     0x7f3057658bb9 - __clone
  16:                0x0 - <unknown>

