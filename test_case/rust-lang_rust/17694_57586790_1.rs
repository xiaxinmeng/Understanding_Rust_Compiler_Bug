
[tom@elbridge ~]$ rustc --version
rustc 0.12.0-dev (0c6679652 2014-09-29 21:12:16 +0000)
[tom@elbridge ~]$ RUST_BACKTRACE=1 rustc foo.rs
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'no entry found for key', /build/rust-git/src/rust/src/libstd/collections/hashmap/map.rs:1093

stack backtrace:
   1:     0x7f6eff51a720 - rt::backtrace::imp::write::ha75ba269c80f79c9UVq
   2:     0x7f6eff51d8b0 - <unknown>
   3:     0x7f6effcd2090 - unwind::begin_unwind_inner::h99aec5f720e41c37SUd
   4:     0x7f6f000a42b0 - <unknown>
   5:     0x7f6f0027f010 - <unknown>
   6:     0x7f6f0027eef0 - <unknown>
   7:     0x7f6f00293040 - <unknown>
   8:     0x7f6f00292d20 - <unknown>
   9:     0x7f6f0028cfd0 - middle::borrowck::gather_loans::gather_loans_in_fn::h7ca090ca6cfae9d2P3h
  10:     0x7f6f002b3310 - <unknown>
  11:     0x7f6f002b07c0 - <unknown>
  12:     0x7f6f002b25b0 - <unknown>
  13:     0x7f6f002b0f70 - middle::borrowck::check_crate::h676a3166f3c656faVGj
  14:     0x7f6f000c1b40 - <unknown>
  15:     0x7f6f008f9670 - driver::driver::phase_3_run_analysis_passes::h7ab4d2823326c4fcaMw
  16:     0x7f6f008f4e80 - driver::driver::compile_input::h47be44a11fe49900Zsw
  17:     0x7f6f00976e10 - <unknown>
  18:     0x7f6f00976cf0 - <unknown>
  19:     0x7f6f000daad0 - <unknown>
  20:     0x7f6f000da8c0 - <unknown>
  21:     0x7f6f01282700 - <unknown>
  22:     0x7f6effd21590 - <unknown>
  23:     0x7f6effd21580 - rust_try
  24:     0x7f6effccf220 - unwind::try::had5a0109a3678a47AJd
  25:     0x7f6effccf0b0 - task::Task::run::h1ae56ff3a9ffb576UYc
  26:     0x7f6f01282470 - <unknown>
  27:     0x7f6effcd1130 - <unknown>
  28:     0x7f6efefe4250 - start_thread
  29:     0x7f6eff9a83b9 - clone
  30:                0x0 - <unknown>
