plain
[01:20:46] travis_fold:end:stage0-linkchecker

[01:20:46] travis_time:end:stage0-linkchecker:start=1536884988639382824,finish=1536884990980139656,duration=2340756832

[01:23:08] std/collections/hash_map/enum.RawEntryMut.html:5: broken link - std/collections/hash_map/struct.Entry.html
[01:24:35] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:24:35] 
[01:24:35] 
[01:24:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:24:35] expected success, got: exit code: 101
[01:24:35] expected success, got: exit code: 101
[01:24:35] 
[01:24:35] 
[01:24:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:35] Build completed unsuccessfully in 0:40:25
[01:24:35] Makefile:58: recipe for target 'check' failed
[01:24:35] make: *** [check] Error 1
141748 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134584 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl/s-f4s12l7dz9-9c2pu6-1jtu7ee7y4raz
134004 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
134000 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
130456 ./obj/cores
126320 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
---
40952 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
39272 ./obj/build/x86_64-unknown-linux-gnu/doc/book
38120 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
37756 ./src/tools/lldb/www
37404 ./obj/brintf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0732e63a
travis_time:start:0732e63a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1080a3a3
$ dmesg | grep -i kill
