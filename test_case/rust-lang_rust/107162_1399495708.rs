
$ LANG=C.utf8 cargo t
   Compiling mozjs_sys v0.68.2 (D:\tmp\mozjs\mozjs)
error: failed to run custom build command for `mozjs_sys v0.68.2 (D:\tmp\mozjs\mozjs)`

Caused by:
  process didn't exit successfully: `D:\tmp\mozjs\target\debug\build\mozjs_sys-352d87b28c39ffac\build-script-build` (exit code: 101)
  --- stdout
  cargo:outdir=D:\tmp\mozjs\target\debug\build\mozjs_sys-27c0928751302e7b\out\build

  --- stderr
  make: invalid --jobserver-auth string '__rust_jobserver_semaphore_2019855554'
  make: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
  D:\tmp\mozjs\mozjs\makefile.cargo:175: *** recipe commences before first target.  Stop.
  thread 'main' panicked at 'assertion failed: result.success()', mozjs\build.rs:178:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
