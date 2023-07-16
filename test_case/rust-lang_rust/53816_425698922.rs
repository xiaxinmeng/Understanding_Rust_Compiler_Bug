plain
[01:49:59]   curl error: Could not resolve host: github.com
[01:49:59] ; class=Net (12)
[01:49:59] 
[01:49:59] 
[01:49:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.4" "cargo-vendor"
[01:49:59] 
[01:49:59] 
[01:49:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:49:59] Build completed unsuccessfully in 1:44:13
---
travis_time:end:07cbe524:start=1538289838920248935,finish=1538289838928305804,duration=8056869
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00755412
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03d11290
travis_time:start:03d11290
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b915993
$ dmesg | grep -i kill
