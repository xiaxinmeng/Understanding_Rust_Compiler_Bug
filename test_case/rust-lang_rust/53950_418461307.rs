plain
[00:49:35] ....................................................................................................
[00:49:39] ....................................................................................................
[00:49:41] ...........................i........................................................................
[00:49:44] ....................................................................................................
[00:49:47] ............................................................................iiiiiiiii...............
[00:49:53] ....................................................................................................
[00:49:57] ....................................................................................................
[00:49:59] .........................................................i..........................................
[00:50:02] ....................................................................................................
---
[01:11:44]    Compiling rustc v0.0.0 (file:///checkout/src/librustc)
[01:12:05] error[E0308]: mismatched types
[01:12:05]     --> librustc/session/config.rs:2968:23
[01:12:05]      |
[01:12:05] 2968 |         opts.cg.lto = Lto::Fat;
[01:12:05]      |                       ^^^^^^^^ expected enum `session::config::LtoCli`, found enum `session::config::Lto`
[01:12:05]      |
[01:12:05]      = note: expected type `session::config::LtoCli`
[01:12:05]                 found type `session::config::Lto`
[01:12:14] error: aborting due to previous error
[01:12:14] 
[01:12:14] For more information about this error, try `rustc --explain E0308`.
[01:12:14] error: Could not compile `rustc`.
[01:12:14] error: Could not compile `rustc`.
[01:12:14] 
[01:12:14] To learn more, run the command again with --verbose.
[01:12:14] 
[01:12:14] 
[01:12:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:12:14] 
[01:12:14] 
[01:12:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:14] Build completed unsuccessfully in 0:26:47
[01:12:14] Build completed unsuccessfully in 0:26:47
[01:12:14] make: *** [check] Error 1
[01:12:14] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:036f1db4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:27a0b62c:start=1536083822240887881,finish=1536083822365883357,duration=124995476
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2649fcd9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0075a638
$ dmesg | grep -i kill
