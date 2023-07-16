plain
travis_time:end:1f455330:start=1561206562652215153,finish=1561206563437283648,duration=785068495
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:18] 
[01:03:18] running 9 tests
[01:03:18] iiiiiiiii
[01:03:18] 
[01:03:18]  finished in 0.145
[01:03:18] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:34] 
[01:03:34] running 122 tests
[01:03:58] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:03] .i.i......iii.i.....ii
[01:04:03] 
[01:04:03]  finished in 29.064
[01:04:03] travis_fold:end:test_debuginfo

---
[01:15:21] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:21]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:15:43] error[E0596]: cannot borrow `*v3` as mutable, as it is behind a `&` reference
[01:15:43]     |
[01:15:43]     |
[01:15:43] 237 |     let v3: &[i32] = &[0, 1, 2, 3, 4];
[01:15:43]     |                      ---------------- help: consider changing this to be a mutable reference: `&mut [0, 1, 2, 3, 4]`
[01:15:43] 238 |     let mut c3 = v3.chunks_mut(10);
[01:15:43]     |                  ^^ `v3` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:15:43] 
[01:15:43] error[E0596]: cannot borrow `*v4` as mutable, as it is behind a `&` reference
[01:15:43]     |
[01:15:43]     |
[01:15:43] 242 |     let v4: &[i32] = &[0, 1, 2];
[01:15:43]     |                      ---------- help: consider changing this to be a mutable reference: `&mut [0, 1, 2]`
[01:15:43] 243 |     let mut c4 = v4.chunks_mut(10);
[01:15:43]     |                  ^^ `v4` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:15:44] error: aborting due to 2 previous errors
[01:15:44] 
[01:15:44] For more information about this error, try `rustc --explain E0596`.
[01:15:45] error: Could not compile `core`.
[01:15:45] error: Could not compile `core`.
[01:15:45] 
[01:15:45] To learn more, run the command again with --verbose.
[01:15:45] 
[01:15:45] 
[01:15:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:15:45] 
[01:15:45] 
[01:15:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:45] Build completed unsuccessfully in 1:11:04
---
travis_time:end:0185e704:start=1561211120845292151,finish=1561211120851389981,duration=6097830
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f61ca13
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bc99454
travis_time:start:0bc99454
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f9b1ea2
$ dmesg | grep -i kill
