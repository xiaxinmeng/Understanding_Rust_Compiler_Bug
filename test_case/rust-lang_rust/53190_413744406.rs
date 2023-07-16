plain
[00:04:24] DirectMap2M:     3078144 kB
[00:04:24] DirectMap1G:    14680064 kB
[00:04:24] + python2.7 ../x.py test
[00:04:24]     Finished dev [unoptimized] target(s) in 0.26s
[00:04:25] thread 'main' panicked at 'fs::read_dir(builder.src.join("src/doc/book/redirects")) failed with No such file or directory (os error 2)', bootstrap/doc.rs:294:21
[00:04:25] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:04:25] Build completed unsuccessfully in 0:00:01
travis_time:end:0d31bd94:start=1534474722562070717,finish=1534474988396638180,duration=265834567463
---
travis_time:end:0381c32e:start=1534474988693627487,finish=1534474988702233260,duration=8605773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09ff6d89
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e08f479
travis_time:start:0e08f479
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1aa19158
$ dmesg | grep -i kill
