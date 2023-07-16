plain
travis_time:end:00bd7d4c:start=1558444897393060849,finish=1558444985219072115,duration=87826011266
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:11] configure: build.locked-deps    := True
[00:07:11] configure: llvm.ccache          := sccache
[00:07:11] configure: build.cargo-native-static := True
[00:07:11] configure: dist.missing-tools   := True
[00:07:11] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:11] configure: writing `config.toml` in current directory
[00:07:11] configure: 
[00:07:11] configure: run `python /checkout/x.py --help`
[00:07:11] configure: 
---
[00:09:07]     Checking hashbrown v0.3.0
[00:09:13] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:09:13]    --> src/libstd/sys/windows/os.rs:290:14
[00:09:13]     |
[00:09:13] 290 |     let me = c::GetCurrentProcess();
[00:09:13]     |              ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[00:09:13]     |
[00:09:13]     = note: consult the function's documentation for information on how to avoid undefined behavior
[00:09:13] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:09:13]    --> src/libstd/sys/windows/os.rs:292:8
[00:09:13]     |
[00:09:13]     |
[00:09:13] 292 |     if c::OpenProcessToken(me, c::TOKEN_READ, &mut token) == 0 {
[00:09:13]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[00:09:13]     |
[00:09:13]     = note: consult the function's documentation for information on how to avoid undefined behavior
[00:09:13] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:09:13]    --> src/libstd/sys/windows/os.rs:297:15
[00:09:13]     |
[00:09:13]     |
[00:09:13] 297 |         match c::GetUserProfileDirectoryW(token, buf, &mut sz) {
[00:09:13]     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[00:09:13]     |
[00:09:13]     = note: consult the function's documentation for information on how to avoid undefined behavior
[00:09:13] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:09:13]    --> src/libstd/sys/windows/os.rs:298:18
[00:09:13]     |
[00:09:13]     |
[00:09:13] 298 |             0 if c::GetLastError() != c::ERROR_INSUFFICIENT_BUFFER => 0,
[00:09:13]     |                  ^^^^^^^^^^^^^^^^^ call to unsafe function
[00:09:13]     |
[00:09:13]     = note: consult the function's documentation for information on how to avoid undefined behavior
[00:09:13] error: aborting due to 4 previous errors
[00:09:13] 
[00:09:13] For more information about this error, try `rustc --explain E0133`.
[00:09:13] error: Could not compile `std`.
---
travis_time:end:02788668:start=1558445549427752178,finish=1558445549432808609,duration=5056431
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ddb99b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03d5ca0c
travis_time:start:03d5ca0c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:041681a8
$ dmesg | grep -i kill
