plain
travis_time:end:20871f72:start=1558729867691488380,finish=1558729868429576633,duration=738088253
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |████████████████████████████████| 542kB 85.1MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 22.8MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.156 (from awscli)
  Downloading https://files.pythonhosted.org/packages/f5/aa/79fc47ccc3c7d0f36aafb9d85091d7d8a8f10d8ad24ccf3a89cf126b9f4e/botocore-1.12.156-py2.py3-none-any.whl (5.4MB)
    0% |▏                               | 20kB 24.0MB/s eta 0:00:01
    0% |▏                               | 30kB 29.8MB/s eta 0:00:01
    0% |▎                               | 40kB 33.2MB/s eta 0:00:01
    0% |▎                               | 51kB 36.8MB/s eta 0:00:01
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:01] 
[01:23:01] running 143 tests
[01:23:04] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:23:06] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:23:06] 
[01:23:06]  finished in 4.678
[01:23:06] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:08] 
[01:23:08] running 9 tests
[01:23:08] iiiiiiiii
[01:23:08] 
[01:23:08]  finished in 0.153
[01:23:08] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:24] 
[01:23:24] running 122 tests
[01:23:50] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:23:55] .i.i......iii.i.....ii
[01:23:55] 
[01:23:55]  finished in 30.734
[01:23:55] travis_fold:end:test_debuginfo

---
[01:33:25]    Doc-tests alloc
[01:33:26] 
[01:33:26] running 380 tests
[01:33:47] .................................................................................................... 100/380
[01:34:00] ............................................................i....................................F.. 200/380
[01:34:24] ................................................................................
[01:34:24] failures:
[01:34:24] 
[01:34:24] ---- rc.rs - rc::Rc<T>::make_mut (line 620) stdout ----
[01:34:24] ---- rc.rs - rc::Rc<T>::make_mut (line 620) stdout ----
[01:34:24] error: unused import: `Weak`
[01:34:24]  --> rc.rs:621:19
[01:34:24]   |
[01:34:24] 4 | use std::rc::{Rc, Weak};
[01:34:24]   |
[01:34:24] note: lint level defined here
[01:34:24]  --> rc.rs:619:9
[01:34:24]   |
---
[01:34:24] 
[01:34:24] error: test failed, to rerun pass '--doc'
[01:34:24] 
[01:34:24] 
[01:34:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:34:24] 
[01:34:24] 
[01:34:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:24] Build completed unsuccessfully in 0:23:27
[01:34:24] Build completed unsuccessfully in 0:23:27
[01:34:24] Makefile:48: recipe for target 'check' failed
[01:34:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:045d3a80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 22:05:43 UTC 2019
---
travis_time:end:1913b473:start=1558735545795552458,finish=1558735545802121040,duration=6568582
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29490ce0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25005ada
travis_time:start:25005ada
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0053f3d0
$ dmesg | grep -i kill
