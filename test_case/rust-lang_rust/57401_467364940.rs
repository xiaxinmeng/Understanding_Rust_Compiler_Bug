plain
travis_time:end:0ae6007c:start=1551167636989679392,finish=1551167709474337710,duration=72484658318
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
[01:18:19] 
[01:18:19] running 119 tests
[01:18:46] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:18:50] i......iii.i.....ii
[01:18:50] 
[01:18:50]  finished in 30.782
[01:18:50] travis_fold:end:test_debuginfo

---
[01:30:02] 
[01:30:02]    Doc-tests core
[01:30:07] 
[01:30:07] running 2278 tests
[01:30:20] ....iiiii.....................................F..................................................... 100/2278
[01:30:47] ..........................................................................................i......... 300/2278
[01:31:03] .................................................................................................... 400/2278
[01:31:17] ......................i..i.......................................................................... 500/2278
[01:31:30] .................................................................................................... 600/2278
---
[01:35:32] ---- cell.rs - cell::DowngradedRef::map (line 1445) stdout ----
[01:35:32] error[E0433]: failed to resolve: use of undeclared type or module `HashMap`
[01:35:32]  --> cell.rs:1450:57
[01:35:32]   |
[01:35:32] 8 | let mut hash: RefCell<HashMap<i32, i32>> = RefCell::new(HashMap::new());
[01:35:32]   |                                                         ^^^^^^^ use of undeclared type or module `HashMap`
[01:35:32] error[E0412]: cannot find type `HashMap` in this scope
[01:35:32]  --> cell.rs:1450:23
[01:35:32]   |
[01:35:32]   |
[01:35:32] 8 | let mut hash: RefCell<HashMap<i32, i32>> = RefCell::new(HashMap::new());
[01:35:32] help: possible candidates are found in other modules, you can import them into scope
[01:35:32]   |
[01:35:32] 5 | use std::collections::HashMap;
[01:35:32]   |
[01:35:32]   |
[01:35:32] 5 | use std::collections::hash_map::HashMap;
[01:35:32]   |
[01:35:32] 
[01:35:32] error[E0599]: no method named `into` found for type `std::cell::RefMut<'_, _>` in the current scope
[01:35:32]   --> cell.rs:1454:39
[01:35:32]    |
[01:35:32] 12 |     |h| h.entry(12).get_or_insert(1)).into());
[01:35:32]    |
[01:35:32]    = note: RefMut::map(
[01:35:32]                hash.borrow_mut(),
[01:35:32]                hash.borrow_mut(),
[01:35:32]                |h| h.entry(12).get_or_insert(1)) is a function, perhaps you wish to call it
[01:35:32] 
[01:35:32] thread 'cell.rs - cell::DowngradedRef::map (line 1445)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:35:32] 
[01:35:32] 
[01:35:32] failures:
[01:35:32]     cell.rs - cell::DowngradedRef::map (line 1445)
[01:35:32]     cell.rs - cell::DowngradedRef::map (line 1445)
[01:35:32] 
[01:35:32] test result: FAILED. 2266 passed; 1 failed; 11 ignored; 0 measured; 0 filtered out
[01:35:32] 
[01:35:32] error: test failed, to rerun pass '--doc'
[01:35:32] 
[01:35:32] 
[01:35:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:35:32] 
[01:35:32] 
[01:35:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:32] Build completed unsuccessfully in 0:29:29
[01:35:32] Build completed unsuccessfully in 0:29:29
[01:35:32] Makefile:48: recipe for target 'check' failed
[01:35:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ee899f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 09:30:52 UTC 2019
---
travis_time:end:14b31100:start=1551173453843659982,finish=1551173453848939845,duration=5279863
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bf594d1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.31523.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 31548]
[New LWP 31551]
[New LWP 31549]
[New LWP 31550]
[New LWP 31523]
[New LWP 31552]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007f12e12305ca in ?? ()
[Current thread is 1 (LWP 31548)]
#0  0x00007f12e12305ca in ?? ()
#1  0x00007f12e4589180 in ?? ()
#2  0x69adb1a58f081300 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_time:end:1bf594d1:start=1551173453854657428,finish=1551173455192947620,duration=1338290192
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:191e400d
travis_time:start:191e400d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22c362c0
$ dmesg | grep -i kill
