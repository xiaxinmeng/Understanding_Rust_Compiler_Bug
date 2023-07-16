plain
[00:07:46]    Compiling cargo-vendor v0.1.4
[00:07:50]     Finished dev [unoptimized + debuginfo] target(s) in 2m 11s
[00:07:50]   Installing /cargo/bin/cargo-vendor
[00:07:50] warning: be sure to add `/cargo/bin` to your PATH to be able to run the installed binaries
[00:07:50] error: could not find `Cargo.lock`, must be run in a directory with Cargo.lock or use the `--sync` option
[00:07:50] To learn more, run the command again with --verbose.
[00:07:50] 
[00:07:50] 
[00:07:50] 
[00:07:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[00:07:50] 
[00:07:50] 
[00:07:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:07:50] Build completed unsuccessfully in 0:04:29
---
travis_time:end:1edc3316:start=1535179677668875999,finish=1535179677685976734,duration=17100735
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2f60cafc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:174e27d7
travis_time:start:174e27d7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06268933
$ dmesg | grep -i kill
