
$ ./x.py test src/test/compile-fail src/test/run-pass
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/liblibc'
Synchronizing submodule url for 'src/llvm'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
HEAD is now at 3bc0272 Merge pull request #26 from TimNN/arm-cc
HEAD is now at e058ca6 Change how the default zone is found
HEAD is now at 7d9b71f Merge pull request #435 from raphlinus/aarch64
HEAD is now at c1d9622 Backport r285278 [ARM] Predicate UMAAL selection on hasDSP.
HEAD is now at a3736a0 Merge pull request #6 from intelfx/patch-1
HEAD is now at 4f99485 Merge pull request #54 from brson/docdir
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_platform_intrinsics v0.0.0 (file:///home/mark/Edit/rust/src/librustc_platform_intrinsics)
   Compiling serialize v0.0.0 (file:///home/mark/Edit/rust/src/libserialize)
   Compiling rustc_llvm v0.0.0 (file:///home/mark/Edit/rust/src/librustc_llvm)
   Compiling syntax_pos v0.0.0 (file:///home/mark/Edit/rust/src/libsyntax_pos)
   Compiling rustc_data_structures v0.0.0 (file:///home/mark/Edit/rust/src/librustc_data_structures)
   Compiling rustc_errors v0.0.0 (file:///home/mark/Edit/rust/src/librustc_errors)
error[E0463]: can't find crate for `term`
  --> src/librustc_errors/lib.rs:30:1
   |
30 | extern crate term;
   | ^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
error: Could not compile `rustc_errors`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/home/mark/Edit/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "8" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" " jemalloc" "--manifest-path" "/home/mark/Edit/rust/src/rustc/Cargo.toml"
expected success, got: exit code: 101
