plain
   Compiling compiler_builtins v0.1.32
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error: cannot find derive macro `Copy` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.75/src/unix/linux_like/linux/gnu/mod.rs:320:18
    |
320 |         #[derive(Copy,Clone)]


error: cannot find derive macro `Clone` in this scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.75/src/unix/linux_like/linux/gnu/mod.rs:320:23
    |
320 |         #[derive(Copy,Clone)]


error[E0658]: unions with non-`Copy` fields are unstable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.75/src/unix/linux_like/linux/gnu/mod.rs:331:9
331 | /         union sifields {
331 | /         union sifields {
332 | |             _align_pointer: *mut ::c_void,
333 | |             sigchld: sifields_sigchld,
    | |_________^
    |
    |
    = note: see issue #55149 <https://github.com/rust-lang/rust/issues/55149> for more information
    = help: add `#![feature(untagged_unions)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
error: could not compile `libc`.
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:01:42
== clock drift check ==
  local time: Thu Aug 20 05:28:30 UTC 2020
  local time: Thu Aug 20 05:28:30 UTC 2020
  network time: Thu, 20 Aug 2020 05:28:30 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6461) (python)
