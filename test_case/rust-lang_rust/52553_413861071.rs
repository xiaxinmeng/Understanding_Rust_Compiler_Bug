plain
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/clippy'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/clang'...
[00:00:20] curl: (52) Empty reply from server
[00:00:20] curl: (52) Empty reply from server
[00:00:20] curl: (52) Empty reply from server
[00:00:21] Command failed. Attempt 2/5:
[00:00:21] Command failed. Attempt 2/5:
[00:00:21] Command failed. Attempt 2/5:
[00:00:21] curl: (52) Empty reply from server
[00:00:21] curl: (52) Empty reply from server
[00:00:21] gzip: stdin: not in gzip format
[00:00:21] tar: Child returned status 1
[00:00:21] tar: Error is not recoverable: exiting now
[00:00:23] Command failed. Attempt 3/5:
[00:00:23] Command failed. Attempt 3/5:
[00:00:23] Command failed. Attempt 3/5:
[00:00:24] curl: (52) Empty reply from server
[00:00:24] curl: (52) Empty reply from server
[00:00:27] Command failed. Attempt 4/5:
[00:00:27] Command failed. Attempt 4/5:
[00:00:27] curl: (52) Empty reply from server
[00:00:27] curl: (52) Empty reply from server
[00:00:31] Command failed. Attempt 5/5:
[00:00:31] Command failed. Attempt 5/5:
[00:00:31] curl: (52) Empty reply from server
[00:00:31] The command has failed after 5 attempts.
[00:00:31] curl: (52) Empty reply from server
[00:00:49] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lld'...
[00:00:49] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lldb'...
[00:00:49] Submodule path 'src/dlmalloc': checked out 'c99638dc2ecfc750cc1656f6edb2bd062c1e0981'
[00:00:49] Submodule path 'src/doc/nomicon': checked out '790e96b87f4b5817cac310e73a524d25c3d076d8'
---
[00:00:55] Submodule path 'src/tools/miri': checked out 'e6f1e15676c26fdc7c4713647fe007b26f361a8e'
[00:00:55] Submodule path 'src/tools/rls': checked out '9e4d8d520390c6aeebc33260026c6ae2946c67ac'
[00:00:56] Submodule path 'src/tools/rust-installer': checked out '27dec6cae3a8132d8a073aad6775425c85095c99'
[00:00:56] Submodule path 'src/tools/rustfmt': checked out '5c9a2b6c13d3b6f8d3f9c02b130bb4b54fd489fb'
[00:01:05] tar: This does not look like a tar archive
[00:01:05] gzip: stdin: not in gzip format
[00:01:05] tar: Child returned status 1
[00:01:05] tar: Error is not recoverable: exiting now
[00:01:05] travis_fold:end:init_repo
---
[00:06:19] DirectMap2M:     3069952 kB
[00:06:19] DirectMap1G:    14680064 kB
[00:06:19] + python2.7 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:06:19]     Finished dev [unoptimized] target(s) in 0.27s
[00:06:21] thread 'main' panicked at 'fs::read_dir(builder.src.join("src/doc/book/redirects")) failed with No such file or directory (os error 2)', bootstrap/doc.rs:294:21
[00:06:21] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:06:21] Build completed unsuccessfully in 0:00:01
travis_time:end:0a9b9f4e:start=1534511120648117324,finish=1534511501931923506,duration=381283806182
---
travis_time:end:02a13f30:start=1534511502220512370,finish=1534511502225709901,duration=5197531
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f332259
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07803f58
travis_time:start:07803f58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:246e08b0
$ dmesg | grep -i kill
