plain
travis_time:end:05b1f6d6:start=1545253659604570469,finish=1545253660711494110,duration=1106923641
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:35]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:03:36] error[E0405]: cannot find trait `TryClone` in this scope
[00:03:36]    --> src/libstd/fs.rs:586:6
[00:03:36]     |
[00:03:36] 586 | impl TryClone for File {
[00:03:36] help: possible candidate is found in another module, you can import it into scope
[00:03:36]     |
[00:03:36] 20  | use core::clone::TryClone;
[00:03:36]     |
[00:03:36]     |
[00:03:36] 
[00:03:36] error[E0405]: cannot find trait `TryClone` in this scope
[00:03:36]    --> src/libstd/net/tcp.rs:579:6
[00:03:36]     |
[00:03:36] 579 | impl TryClone for TcpStream {
[00:03:36] help: possible candidate is found in another module, you can import it into scope
[00:03:36]     |
[00:03:36] 11  | use core::clone::TryClone;
[00:03:36]     |
[00:03:36]     |
[00:03:36] 
[00:03:36] error[E0405]: cannot find trait `TryClone` in this scope
[00:03:36]    --> src/libstd/net/tcp.rs:899:6
[00:03:36]     |
[00:03:36] 899 | impl TryClone for TcpListener {
[00:03:36] help: possible candidate is found in another module, you can import it into scope
[00:03:36]     |
[00:03:36] 11  | use core::clone::TryClone;
[00:03:36]     |
[00:03:36]     |
[00:03:36] 
[00:03:36] error[E0405]: cannot find trait `TryClone` in this scope
[00:03:36]    --> src/libstd/net/udp.rs:801:6
[00:03:36]     |
[00:03:36] 801 | impl TryClone for UdpSocket {
[00:03:36] help: possible candidate is found in another module, you can import it into scope
[00:03:36]     |
[00:03:36] 11  | use core::clone::TryClone;
[00:03:36]     |
---
[00:03:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:36] expected success, got: exit code: 101
[00:03:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:36] Build completed unsuccessfully in 0:00:40
[00:03:36] make: *** [all] Error 1
[00:03:36] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:031103a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 21:11:26 UTC 2018
---
travis_time:end:0dce90a8:start=1545253886906345854,finish=1545253886912995596,duration=6649742
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1597aa03
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:061e49ae
travis_time:start:061e49ae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02dcfad0
$ dmesg | grep -i kill
