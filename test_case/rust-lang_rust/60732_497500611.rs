plain
travis_time:end:04b228c2:start=1559247454644545550,finish=1559247457528006407,duration=2883460857
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:25] 
[01:17:25] running 143 tests
[01:17:27] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:17:29] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:17:29] 
[01:17:29]  finished in 4.532
[01:17:29] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:31] 
[01:17:31] running 9 tests
[01:17:31] iiiiiiiii
[01:17:31] 
[01:17:31]  finished in 0.149
[01:17:31] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:48] 
[01:17:48] running 122 tests
[01:18:12] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:18:17] .i.i......iii.i.....ii
[01:18:17] 
[01:18:17]  finished in 29.301
[01:18:17] travis_fold:end:test_debuginfo

---
[01:39:40] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0730 (line 11759) stdout ----
[01:39:40] error[E0411]: cannot find type `Self` in this scope
[01:39:40]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11773:28
[01:39:40]    |
[01:39:40] 16 |     unsafe { *(v as *const Self as *const u8) }
[01:39:40]    |                            ^^^^ `Self` is only available in impls, traits, and type definitions
[01:39:40] error[E0308]: mismatched types
[01:39:40]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11776:28
[01:39:40]    |
[01:39:40]    |
[01:39:40] 19 | assert_eq!(3, discriminant(Enum::Unit));
[01:39:40]    |                            ^^^^^^^^^^ expected &main::Enum, found enum `main::Enum`
[01:39:40]    = note: expected type `&main::Enum`
[01:39:40]               found type `main::Enum`
[01:39:40] 
[01:39:40] error[E0308]: mismatched types
[01:39:40] error[E0308]: mismatched types
[01:39:40]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11777:28
[01:39:40]    |
[01:39:40] 20 | assert_eq!(2, discriminant(Enum::Tuple(5)));
[01:39:40]    |                            ^^^^^^^^^^^^^^ expected &main::Enum, found enum `main::Enum`
[01:39:40]    = note: expected type `&main::Enum`
[01:39:40]               found type `main::Enum`
[01:39:40] 
[01:39:40] error[E0308]: mismatched types
[01:39:40] error[E0308]: mismatched types
[01:39:40]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11778:28
[01:39:40]    |
[01:39:40] 21 | assert_eq!(1, discriminant(Enum::Struct{a: 7, b: 11}));
[01:39:40]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ expected &main::Enum, found enum `main::Enum`
[01:39:40]    = note: expected type `&main::Enum`
[01:39:40]               found type `main::Enum`
[01:39:40] 
[01:39:40] error: aborting due to 4 previous errors
---
[01:39:40] 
[01:39:40] 
[01:39:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:40] Build completed unsuccessfully in 0:33:35
[01:39:40] make: *** [check] Error 1
[01:39:40] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18d712ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 21:57:28 UTC 2019
---
travis_time:end:04f821a8:start=1559253450461851320,finish=1559253450467181074,duration=5329754
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00185400
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:163b1500
travis_time:start:163b1500
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:046d6df8
$ dmesg | grep -i kill
