plain
travis_time:end:08d21196:start=1549025205190868181,finish=1549025206096893701,duration=906025520
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:42] 
[01:16:42] running 119 tests
[01:17:08] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:17:12] i......iii.i.....ii
[01:17:12] 
[01:17:12]  finished in 30.297
[01:17:12] travis_fold:end:test_debuginfo

---
[01:24:24]    Compiling rand_chacha v0.1.0
[01:24:24]    Compiling rand_pcg v0.1.1
[01:24:24]    Compiling rand v0.6.1
[01:24:29]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:24:30] error[E0658]: use of unstable library feature 'fn_traits' (see issue #29625)
[01:24:30]     |
[01:24:30]     |
[01:24:30] 614 | / impl_fn_for_zst! {
[01:24:30] 615 | |     #[derive(Clone)]
[01:24:30] 616 | |     struct CharEscapeDebugContinue impl Fn = |c: char| -> char::EscapeDebug {
[01:24:30] 617 | |         c.escape_debug_ext(false)
[01:24:30] 627 | |     };
[01:24:30] 628 | | }
[01:24:30]     | |_^
[01:24:30]     |
---
[01:24:33] warning: build failed, waiting for other jobs to finish...
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:982:5
[01:24:34]     |
[01:24:34] 982 |     assert_eq!("abc".escape_unicode(), "\\u{61}\\u{62}\\u{63}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:983:5
[01:24:34]     |
[01:24:34] 983 |     assert_eq!("a c".escape_unicode(), "\\u{61}\\u{20}\\u{63}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:984:5
[01:24:34]     |
[01:24:34] 984 |     assert_eq!("\r\n\t".escape_unicode(), "\\u{d}\\u{a}\\u{9}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:985:5
[01:24:34]     |
[01:24:34] 985 |     assert_eq!("'\"\\".escape_unicode(), "\\u{27}\\u{22}\\u{5c}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:986:5
[01:24:34]     |
[01:24:34] 986 |     assert_eq!("\x00\x01\u{fe}\u{ff}".escape_unicode(), "\\u{0}\\u{1}\\u{fe}\\u{ff}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:987:5
[01:24:34]     |
[01:24:34] 987 |     assert_eq!("\u{100}\u{ffff}".escape_unicode(), "\\u{100}\\u{ffff}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:988:5
[01:24:34]     |
[01:24:34] 988 |     assert_eq!("\u{10000}\u{10ffff}".escape_unicode(), "\\u{10000}\\u{10ffff}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:989:5
[01:24:34]     |
[01:24:34] 989 |     assert_eq!("ab\u{fb00}".escape_unicode(), "\\u{61}\\u{62}\\u{fb00}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeUnicode<'_>`
[01:24:34]    --> src/liballoc/../liballoc/tests/str.rs:990:5
[01:24:34]     |
[01:24:34] 990 |     assert_eq!("\u{1d4ea}\r".escape_unicode(), "\\u{1d4ea}\\u{d}");
[01:24:34]     |
[01:24:34]     = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeUnicode<'_>`
[01:24:34]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1001 |     assert_eq!("abc".escape_debug(), "abc");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1002 |     assert_eq!("a c".escape_debug(), "a c");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1003 |     assert_eq!("éèê".escape_debug(), "éèê");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1004 |     assert_eq!("\r\n\t".escape_debug(), "\\r\\n\\t");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1005 |     assert_eq!("'\"\\".escape_debug(), "\\'\\\"\\\\");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1006 |     assert_eq!("\u{7f}\u{ff}".escape_debug(), "\\u{7f}\u{ff}");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1007 |     assert_eq!("\u{100}\u{ffff}".escape_debug(), "\u{100}\\u{ffff}");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1008 |     assert_eq!("\u{10000}\u{10ffff}".escape_debug(), "\u{10000}\\u{10ffff}");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1009 |     assert_eq!("ab\u{200b}".escape_debug(), "ab\\u{200b}");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1010 |     assert_eq!("\u{10d4ea}\r".escape_debug(), "\\u{10d4ea}\\r");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDebug<'_>`
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1011 |     assert_eq!("\u{301}a\u{301}bé\u{e000}".escape_debug(), "\\u{301}a\u{301}bé\\u{e000}");
[01:24:34]      |
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDebug<'_>`
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1016:5
[01:24:34]      |
[01:24:34]      |
[01:24:34] 1016 |     assert_eq!("abc".escape_default(), "abc");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1017:5
[01:24:34]      |
[01:24:34] 1017 |     assert_eq!("a c".escape_default(), "a c");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1018:5
[01:24:34]      |
[01:24:34] 1018 |     assert_eq!("éèê".escape_default(), "\\u{e9}\\u{e8}\\u{ea}");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1019:5
[01:24:34]      |
[01:24:34] 1019 |     assert_eq!("\r\n\t".escape_default(), "\\r\\n\\t");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1020:5
[01:24:34]      |
[01:24:34] 1020 |     assert_eq!("'\"\\".escape_default(), "\\'\\\"\\\\");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1021:5
[01:24:34]      |
[01:24:34] 1021 |     assert_eq!("\u{7f}\u{ff}".escape_default(), "\\u{7f}\\u{ff}");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1022:5
[01:24:34]      |
[01:24:34] 1022 |     assert_eq!("\u{100}\u{ffff}".escape_default(), "\\u{100}\\u{ffff}");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1023:5
[01:24:34]      |
[01:24:34] 1023 |     assert_eq!("\u{10000}\u{10ffff}".escape_default(), "\\u{10000}\\u{10ffff}");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1024:5
[01:24:34]      |
[01:24:34] 1024 |     assert_eq!("ab\u{200b}".escape_default(), "ab\\u{200b}");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
[01:24:34] 
[01:24:34] error[E0369]: binary operation `==` cannot be applied to type `std::str::EscapeDefault<'_>`
[01:24:34]     --> src/liballoc/../liballoc/tests/str.rs:1025:5
[01:24:34]      |
[01:24:34] 1025 |     assert_eq!("\u{10d4ea}\r".escape_default(), "\\u{10d4ea}\\r");
[01:24:34]      |
[01:24:34]      = note: an implementation of `std::cmp::PartialEq` might be missing for `std::str::EscapeDefault<'_>`
[01:24:34]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:24:34] 
---
[01:24:35] 
[01:24:35] To learn more, run the command again with --verbose.
[01:24:35] 
[01:24:35] 
[01:24:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:24:35] 
[01:24:35] 
[01:24:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:35] Build completed unsuccessfully in 0:19:46
[01:24:35] Build completed unsuccessfully in 0:19:46
[01:24:36] Makefile:48: recipe for target 'check' failed
[01:24:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00dd6720
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 14:11:33 UTC 2019
---
travis_time:end:0215996e:start=1549030295255233183,finish=1549030295261783461,duration=6550278
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0824c2d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14f69328
travis_time:start:14f69328
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09a0faa5
$ dmesg | grep -i kill
