
 ---- build::close_output stdout ----
thread 'build::close_output' panicked at 'lines differ:
   Compiling foo v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t216/foo)
error: Broken pipe (os error 32)
warning: build failed, waiting for other jobs to finish...
hello stderr!
error: build failed
', src/tools/cargo/tests/testsuite/build.rs:5043:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    build::close_output
