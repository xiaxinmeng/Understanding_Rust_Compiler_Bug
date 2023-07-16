plain
[01:37:07] Build completed successfully in 1:32:25
[01:37:07] + python2.7 ../x.py dist --target i586-unknown-linux-gnu,i686-unknown-linux-musl,i586-unknown-linux-musl
[01:37:07]     Finished dev [unoptimized] target(s) in 0.26s
[01:37:28] fatal: unable to access 'https://github.com/rust-lang/rust.git/': Could not resolve host: github.com
[01:37:28] thread 'main' panicked at 'command did not execute successfully: "git" "ls-remote" "origin" "beta"
[01:37:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:37:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target i586-unknown-linux-gnu,i686-unknown-linux-musl,i586-unknown-linux-musl
[01:37:28] Build completed unsuccessfully in 0:00:21
travis_time:end:1ab00596:start=1537334197980408234,finish=1537340047008730856,duration=5849028322622
---
travis_time:end:059f0cca:start=1537340048621225390,finish=1537340048625452341,duration=4226951
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b6b835e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0077e707
travis_time:start:0077e707
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b1beb60
$ dmesg | grep -i kill
