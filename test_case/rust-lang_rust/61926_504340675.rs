plain
travis_time:end:3005a9b0:start=1561099211084249740,finish=1561099211912944869,duration=828695129
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
[01:09:58] 
[01:09:58] running 9 tests
[01:09:58] iiiiiiiii
[01:09:58] 
[01:09:58]  finished in 0.164
[01:09:58] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:15] 
[01:10:15] running 122 tests
[01:10:42] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:10:47] .i.i......iii.i.....ii
[01:10:47] 
[01:10:47]  finished in 32.060
[01:10:47] travis_fold:end:test_debuginfo

---
[01:48:28] travis_fold:end:stage0-linkchecker

[01:48:28] travis_time:end:stage0-linkchecker:start=1561105728320341340,finish=1561105730746704722,duration=2426363382

[01:48:30] std/vec/struct.Vec.html:2096: broken link - std/vec/VecDeque
[01:48:30] std/vec/struct.Vec.html:2135: broken link - std/vec/VecDeque
[01:48:31] std/collections/struct.VecDeque.html:633: broken link - std/collections/VecDeque
[01:48:31] std/collections/struct.VecDeque.html:655: broken link - std/collections/VecDeque
[01:48:31] std/collections/vec_deque/struct.VecDeque.html:633: broken link - std/collections/vec_deque/VecDeque
[01:48:31] std/collections/vec_deque/struct.VecDeque.html:655: broken link - std/collections/vec_deque/VecDeque
[01:48:32] std/convert/trait.From.html:625: broken link - std/convert/VecDeque
[01:48:32] std/convert/trait.From.html:647: broken link - std/convert/VecDeque
[01:48:35] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:48:35] 
[01:48:35] 
[01:48:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:48:35] expected success, got: exit code: 101
---
travis_time:end:1d4fccf9:start=1561105740313429451,finish=1561105740320390683,duration=6961232
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03dd7b40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21e48df0
travis_time:start:21e48df0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b0eb565
$ dmesg | grep -i kill
