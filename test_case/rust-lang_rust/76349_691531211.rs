plain
[100%] Built target clang_rt.tsan-x86_64
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers
 finished in 15.139
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit code: 127)
--- stderr
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc: error while loading shared libraries: libLLVM-11-rust-1.48.0-nightly.so: cannot open shared object file: No such file or directory

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:13:19
== clock drift check ==
  local time: Sat Sep 12 18:56:51 UTC 2020
  local time: Sat Sep 12 18:56:51 UTC 2020
  network time: Sat, 12 Sep 2020 18:56:51 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4782) (python)
