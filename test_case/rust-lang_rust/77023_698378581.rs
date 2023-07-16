plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.35
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
error[E0636]: the feature `const_fn` has already been declared
   |
85 | #![feature(const_fn)]
   |            ^^^^^^^^

---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:01:18
== clock drift check ==
  local time: Thu Sep 24 14:24:23 UTC 2020
  local time: Thu Sep 24 14:24:23 UTC 2020
  network time: Thu, 24 Sep 2020 14:24:23 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4743) (python)
