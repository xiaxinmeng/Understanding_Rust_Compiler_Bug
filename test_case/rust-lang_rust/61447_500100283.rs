plain
travis_time:end:064df74e:start=1559968781753484728,finish=1559968869109880748,duration=87356396020
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
[01:07:11] 
[01:07:11] running 144 tests
[01:07:14] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:07:16] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:07:16] 
[01:07:16]  finished in 4.825
[01:07:16] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:18] 
[01:07:18] running 9 tests
[01:07:18] iiiiiiiii
[01:07:18] 
[01:07:18]  finished in 0.157
[01:07:18] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:35] 
[01:07:35] running 122 tests
[01:08:01] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:08:06] .i.i......iii.i.....ii
[01:08:06] 
[01:08:06]  finished in 31.610
[01:08:06] travis_fold:end:test_debuginfo

---
[01:46:22] travis_fold:end:stage0-linkchecker

[01:46:22] travis_time:end:stage0-linkchecker:start=1559975258428390560,finish=1559975260691941004,duration=2263550444

[01:46:22] alloc/vec/struct.Vec.html:804: broken link - alloc/collections/struct.VecDeque.html
[01:46:22] alloc/vec/struct.Vec.html:827: broken link - alloc/collections/struct.VecDeque.html
[01:46:22] alloc/collections/vec_deque/struct.VecDeque.html:629: broken link - alloc/collections/vec/struct.Vec.html
[01:46:22] alloc/collections/vec_deque/struct.VecDeque.html:629: broken link - alloc/collections/collections/struct.VecDeque.html
[01:46:22] alloc/collections/vec_deque/struct.VecDeque.html:652: broken link - alloc/collections/collections/struct.VecDeque.html
[01:46:22] alloc/collections/vec_deque/struct.VecDeque.html:652: broken link - alloc/collections/vec/struct.Vec.html
[01:46:26] std/collections/vec_deque/struct.VecDeque.html:649: broken link - std/collections/collections/struct.VecDeque.html
[01:46:26] std/collections/vec_deque/struct.VecDeque.html:649: broken link - std/collections/vec/struct.Vec.html
[01:46:26] std/collections/vec_deque/struct.VecDeque.html:671: broken link - std/collections/vec/struct.Vec.html
[01:46:26] std/collections/vec_deque/struct.VecDeque.html:671: broken link - std/collections/collections/struct.VecDeque.html
[01:46:30] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:46:30] 
[01:46:30] 
[01:46:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:46:30] expected success, got: exit code: 101
---
travis_time:end:1a57f4aa:start=1559975271489442425,finish=1559975271496417216,duration=6974791
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06120934
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e3584ec
travis_time:start:0e3584ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:204480c0
$ dmesg | grep -i kill
