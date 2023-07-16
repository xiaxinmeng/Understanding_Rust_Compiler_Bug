plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:42:59] 
[00:42:59] running 1504 tests
[00:43:03] ........F....................................................................................i......
[00:43:13] ....................................................................................................
[00:43:16] ....................................................................................................
[00:43:20] ....................................................................................................
[00:43:23] ....................................................................................................
---
[00:44:12] ...................i................................................................................
re.3
travis_fold:start:after_failure.4
travis_time:start:35547f60
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:269c87b5
$ dmesg | grep -i kill
