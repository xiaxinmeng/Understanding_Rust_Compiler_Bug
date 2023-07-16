plain
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.320
[RUSTC-TIMING] build_script_build test:false 0.344
[RUSTC-TIMING] build_script_build test:false 0.513
error[E0407]: method `get_unchecked` is not a member of trait `TrustedRandomAccess`
     |
     |
6062 | /     unsafe fn get_unchecked(&mut self, i: usize) -> &'a mut [T; N] {
6063 | |         unsafe { self.iter.get_unchecked(i) }
6064 | |     }
     | |_____^ not a member of trait `TrustedRandomAccess`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0407`.
[RUSTC-TIMING] core test:false 8.219
[RUSTC-TIMING] core test:false 8.219
error: could not compile `core`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
Build completed unsuccessfully in 0:00:09
== clock drift check ==
  local time: Sat Sep  5 02:22:39 UTC 2020
  local time: Sat Sep  5 02:22:39 UTC 2020
  network time: Sat, 05 Sep 2020 02:22:39 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4048) (node)
Terminate orphan process: pid (4057) (python)
