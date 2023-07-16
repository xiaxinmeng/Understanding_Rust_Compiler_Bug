plain
travis_time:end:13a2133a:start=1553487148706837675,finish=1553487222355491777,duration=73648654102
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:46] 
[01:22:46] running 9 tests
[01:22:46] iiiiiiiii
[01:22:46] 
[01:22:46]  finished in 0.174
[01:22:46] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:03] 
[01:23:03] running 120 tests
[01:23:29] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:23:34] .i......iii.i.....ii
[01:23:34] 
[01:23:34]  finished in 31.254
[01:23:34] travis_fold:end:test_debuginfo

---
[01:44:23]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:44:49] error[E0308]: mismatched types
[01:44:49]     --> src/librustc/session/config.rs:2826:29
[01:44:49]      |
[01:44:49] 2826 |                 mk_set(vec![Some(String::from("b")), Some(String::from("c"))]),
[01:44:49]      |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `session::config::ExternEntry`, found enum `std::option::Option`
[01:44:49]      = note: expected type `session::config::ExternEntry`
[01:44:49]                 found type `std::option::Option<std::string::String>`
[01:44:49] 
[01:44:49] error[E0308]: mismatched types
[01:44:49] error[E0308]: mismatched types
[01:44:49]     --> src/librustc/session/config.rs:2830:29
[01:44:49]      |
[01:44:49] 2830 |                 mk_set(vec![Some(String::from("e")), Some(String::from("f"))]),
[01:44:49]      |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `session::config::ExternEntry`, found enum `std::option::Option`
[01:44:49]      = note: expected type `session::config::ExternEntry`
[01:44:49]                 found type `std::option::Option<std::string::String>`
[01:44:49] 
[01:44:49] error[E0308]: mismatched types
[01:44:49] error[E0308]: mismatched types
[01:44:49]     --> src/librustc/session/config.rs:2837:29
[01:44:49]      |
[01:44:49] 2837 |                 mk_set(vec![Some(String::from("e")), Some(String::from("f"))]),
[01:44:49]      |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `session::config::ExternEntry`, found enum `std::option::Option`
[01:44:49]      = note: expected type `session::config::ExternEntry`
[01:44:49]                 found type `std::option::Option<std::string::String>`
[01:44:49] 
[01:44:49] error[E0308]: mismatched types
[01:44:49] error[E0308]: mismatched types
[01:44:49]     --> src/librustc/session/config.rs:2841:29
[01:44:49]      |
[01:44:49] 2841 |                 mk_set(vec![Some(String::from("b")), Some(String::from("c"))]),
[01:44:49]      |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `session::config::ExternEntry`, found enum `std::option::Option`
[01:44:49]      = note: expected type `session::config::ExternEntry`
[01:44:49]                 found type `std::option::Option<std::string::String>`
[01:44:49] 
[01:44:49] error[E0308]: mismatched types
[01:44:49] error[E0308]: mismatched types
[01:44:49]     --> src/librustc/session/config.rs:2848:29
[01:44:49]      |
[01:44:49] 2848 |                 mk_set(vec![Some(String::from("b")), Some(String::from("c"))]),
[01:44:49]      |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `session::config::ExternEntry`, found enum `std::option::Option`
[01:44:49]      = note: expected type `session::config::ExternEntry`
[01:44:49]                 found type `std::option::Option<std::string::String>`
[01:44:49] 
[01:44:49] error[E0308]: mismatched types
[01:44:49] error[E0308]: mismatched types
[01:44:49]     --> src/librustc/session/config.rs:2852:29
[01:44:49]      |
[01:44:49] 2852 |                 mk_set(vec![Some(String::from("f")), Some(String::from("e"))]),
[01:44:49]      |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `session::config::ExternEntry`, found enum `std::option::Option`
[01:44:49]      = note: expected type `session::config::ExternEntry`
[01:44:49]                 found type `std::option::Option<std::string::String>`
[01:44:49] 
[01:44:59] error: aborting due to 6 previous errors
[01:44:59] error: aborting due to 6 previous errors
[01:44:59] 
[01:44:59] For more information about this error, try `rustc --explain E0308`.
[01:44:59] error: Could not compile `rustc`.
[01:44:59] 
[01:44:59] To learn more, run the command again with --verbose.
[01:44:59] 
[01:44:59] 
[01:44:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:44:59] 
[01:44:59] 
[01:44:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:59] Build completed unsuccessfully in 0:34:06
[01:44:59] Build completed unsuccessfully in 0:34:06
[01:44:59] Makefile:48: recipe for target 'check' failed
[01:44:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04edf182
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 05:58:51 UTC 2019
---
travis_time:end:05bb51b8:start=1553493533303017648,finish=1553493533309642338,duration=6624690
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ffffb4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b528f1a
travis_time:start:2b528f1a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ccbe73c
$ dmesg | grep -i kill
