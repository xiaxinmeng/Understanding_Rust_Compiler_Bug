plain
travis_time:end:02e3c79d:start=1556395233741913390,finish=1556395234554442248,duration=812528858
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:34]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:36]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:36]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:36]    Compiling rustc-demangle v0.1.10
[00:04:37] error[E0592]: duplicate definitions with name `capacity`
[00:04:37]     |
[00:04:37] 96  | /     fn capacity(&self) -> usize {
[00:04:37] 97  | |         if mem::size_of::<T>() == 0 {
[00:04:37] 97  | |         if mem::size_of::<T>() == 0 {
[00:04:37] 98  | |             // For zero sized types, we are always at maximum capacity
[00:04:37] 99  | |             MAXIMUM_ZST_CAPACITY
[00:04:37] 102 | |         }
[00:04:37] 103 | |     }
[00:04:37] 103 | |     }
[00:04:37]     | |_____^ duplicate definitions for `capacity`
[00:04:37] 509 | /     pub fn capacity(&self) -> usize {
[00:04:37] 510 | |         self.capacity() - 1
[00:04:37] 511 | |     }
[00:04:37]     | |_____- other definition for `capacity`
---
travis_time:end:23d3fe40:start=1556395524360969314,finish=1556395524365695278,duration=4725964
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:102b1246
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0044588a
travis_time:start:0044588a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10950300
$ dmesg | grep -i kill
