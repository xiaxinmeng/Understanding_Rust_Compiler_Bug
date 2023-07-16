plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/78357/merge:refs/remotes/pull/78357/merge
---
  Downloaded compiler_builtins v0.1.36
   Compiling cc v1.0.60
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.79
   Compiling libc v0.2.79 (https://github.com/lzutao/rust-libc?branch=i78184#b58a839d)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-freebsd)
   Compiling cc v1.0.60
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.79
   Compiling libc v0.2.79 (https://github.com/lzutao/rust-libc?branch=i78184#b58a839d)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error[E0412]: cannot find type `Option` in this scope
    --> /cargo/git/checkouts/rust-libc-281fa28986f301e1/b58a839/src/unix/bsd/freebsdlike/mod.rs:1589:19
1589 |         callback: Option<
     |                   ^^^^^^ not found in this scope
     |
help: consider importing this enum
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-freebsd" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
Build completed unsuccessfully in 0:07:48
== clock drift check ==
  local time: Mon Oct 26 02:59:06 UTC 2020
  local time: Mon Oct 26 02:59:06 UTC 2020
  network time: Sun, 25 Oct 2020 21:05:23 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3437) (python)
