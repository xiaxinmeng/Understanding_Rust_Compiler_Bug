plain
   Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
  --> src/tools/rls/rls-rustc/src/lib.rs:78:41
   |
78 |     rustc_driver::catch_fatal_errors(|| run_compiler(&args, &mut shim_calls, file_loader, None))
   |                                         |
   |                                         expected 5 arguments

error: aborting due to previous error
---
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'Unable to build RLS', src/bootstrap/dist.rs:48:9
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:35:19
== clock drift check ==
  local time: Tue Sep 29 20:03:47 UTC 2020
  local time: Tue Sep 29 20:03:47 UTC 2020
  network time: Tue, 29 Sep 2020 20:03:47 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2973) (python)
