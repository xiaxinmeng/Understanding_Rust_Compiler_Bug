plain
[00:48:58] ....................................................................................................
[00:49:01] ................................................................i...................................
[00:49:04] ....................................................................................................
[00:49:07] ....................................................................................................
[00:49:10] .............iiiiiiiii..............................................................................
[00:49:16] ....................................................................................................
[00:49:20] .................................................................................................i..
[00:49:23] ....................................................................................................
[00:49:26] .........................................................i.i..ii....................................
---
[01:31:36] 
[01:31:36] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:31:36] diff of stderr:
[01:31:36] 
[01:31:36] - warning: `[Foo::baz]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:31:36] -    |
[01:31:36] -    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:31:36] -    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[Bar::foo]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |      //! , [Uniooon::X] and [Qux::Z].
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |      //! , [Uniooon::X] and [Qux::Z].
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |       //! , [Uniooon::X] and [Qux::Z].
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |       //! , [Uniooon::X] and [Qux::Z].
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[Qux:Y]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |        /// [Qux:Y]
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[BarA]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL | /// bar [BarA] bar
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[BarB]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL | / /**
[01:31:36] - LL | |  * Foo
[01:31:36] - LL | |  * bar [BarB] bar
[01:31:36] - LL | |  * baz
[01:31:36] - LL | |  */
[01:31:36] -    | |___^
[01:31:36] -    = note: the link appears in this line:
[01:31:36] -            
[01:31:36] -            
[01:31:36] -             bar [BarB] bar
[01:31:36] -                  ^^^^
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[BarC]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL | / /** Foo
[01:31:36] - LL | |
[01:31:36] - LL | | bar [BarC] bar
[01:31:36] - LL | | baz
[01:31:36] - ...  |
[01:31:36] - LL | |
[01:31:36] - LL | | */
[01:31:36] -    |
[01:31:36] -    = note: the link appears in this line:
[01:31:36] -            
[01:31:36] -            
[01:31:36] -            bar [BarC] bar
[01:31:36] -                 ^^^^
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[BarD]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL | #[doc = "Foo/nbar [BarD] bar/nbaz"]
[01:31:36] -    |
[01:31:36] -    = note: the link appears in this line:
[01:31:36] -            
[01:31:36] -            
[01:31:36] -            bar [BarD] bar
[01:31:36] -                 ^^^^
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] - warning: `[BarF]` cannot be resolved, ignoring it...
[01:31:36] -    |
[01:31:36] -    |
[01:31:36] - LL |         #[doc = $f]
[01:31:36] - ...
[01:31:36] - ...
[01:31:36] - LL | f!("Foo/nbar [BarF] bar/nbaz");
[01:31:36] -    | ------------------------------- in this macro invocation
[01:31:36] -    = note: the link appears in this line:
[01:31:36] -            
[01:31:36] -            
[01:31:36] -            bar [BarF] bar
[01:31:36] -                 ^^^^
[01:31:36] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:31:36] - 
[01:31:36] 
[01:31:36] 
[01:31:36] 
[01:31:36] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr`: No such file or directory (os error 2)
[01:31:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:31:36] 
[01:31:36] 
[01:31:36] failures:
[01:31:36] failures:
[01:31:36]     [ui] rustdoc-ui/intra-links-warning.rs
[01:31:36] 
[01:31:36] test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:31:36] 
[01:31:36] 
[01:31:36] 
[01:31:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:31:36] 
[01:31:36] 
[01:31:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:36] Build completed unsuccessfully in 0:46:51
[01:31:36] Build completed unsuccessfully in 0:46:51
[01:31:36] Makefile:58: recipe for target 'check' failed
[01:31:36] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3b91e866
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
