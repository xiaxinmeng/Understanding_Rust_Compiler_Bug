plain
[00:06:52] DirectMap2M:     3080192 kB
[00:06:52] DirectMap1G:    14680064 kB
[00:06:52] + python2.7 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:06:53]     Finished dev [unoptimized] target(s) in 0.26s
[00:06:54] thread 'main' panicked at 'fs::read_dir(builder.src.join("src/doc/book/redirects")) failed with No such file or directory (os error 2)', bootstrap/doc.rs:294:21
[00:06:54] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:06:54] Build completed unsuccessfully in 0:00:01
travis_time:end:12bacf2d:start=1534475150822412395,finish=1534475565198225591,duration=414375813196
---
travis_time:end:1db66dde:start=1534475565580410264,finish=1534475565586992872,duration=6582608
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e47eef3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06817514
travis_time:start:06817514
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2ec50381
$ dmesg | grep -i kill
