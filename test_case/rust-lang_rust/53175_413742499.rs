plain
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:00] tar: This does not look like a tar archive
[00:00:00] gzip: stdin: not in gzip format
[00:00:00] tar: Child returned status 1
[00:00:00] tar: Error is not recoverable: exiting now
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/nomicon'...
---
[00:03:29] DirectMap2M:     3072000 kB
[00:03:29] DirectMap1G:    14680064 kB
[00:03:29] + python2.7 ../x.py test
[00:03:29]     Finished dev [unoptimized] target(s) in 0.26s
[00:03:31] thread 'main' panicked at 'fs::read_dir(builder.src.join("src/doc/book/redirects")) failed with No such file or directory (os error 2)', bootstrap/doc.rs:294:21
[00:03:31] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:03:31] Build completed unsuccessfully in 0:00:01
travis_time:end:02865f0e:start=1534473972679144640,finish=1534474183996299688,duration=211317155048
---
travis_time:end:0394a9f7:start=1534474184465389914,finish=1534474184473010174,duration=7620260
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:062caf9e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23d1ba5c
travis_time:start:23d1ba5c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01267d25
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.565322] init: failsafe main process (1097) killed by TERM signal
[   41.972699] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
