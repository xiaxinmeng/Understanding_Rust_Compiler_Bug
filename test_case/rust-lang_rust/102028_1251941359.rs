
error: failed to run custom build command for `libffi-sys v2.0.0`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/build/libffi-sys-0c4808e136dd5138/build-script-build` (exit status: 101)
  --- stdout
  checking build system type... x86_64-pc-linux-gnu
  checking host system type... 
  --- stderr
  Invalid configuration `i686-pc-windows-gnu': Kernel `windows' not known to work with OS `gnu'.
  configure: error: /bin/bash ./config.sub i686-pc-windows-gnu failed
  thread 'main' panicked at 'Configuring libffi', /cargo/registry/src/github.com-1ecc6299db9ec823/libffi-sys-2.0.0/build/common.rs:8:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:02:19
== clock drift check ==
  local time: Tue Sep 20 07:13:47 UTC 2022
  network time: Tue, 20 Sep 2022 07:13:47 GMT
== end clock drift check ==
Error: Process completed with exit code 1.
