plain
travis_time:end:25503d0c:start=1561132946706188535,finish=1561132947517231993,duration=811043458
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:24]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:28]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:28]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:38]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:09] error[E0004]: non-exhaustive patterns: `InvalidIntToPtrCast` not covered
[00:09:09]    --> src/librustc/mir/interpret/error.rs:324:15
[00:09:09] 228 | / pub enum InterpError<'tcx, O> {
[00:09:09] 228 | / pub enum InterpError<'tcx, O> {
[00:09:09] 229 | |     /// This variant is used by machines to signal their own errors that do not
[00:09:09] 230 | |     /// match an existing variant.
[00:09:09] 231 | |     MachineError(String),
[00:09:09] ...   |
[00:09:09] 316 | |     InvalidIntToPtrCast,
[00:09:09]     | |     ------------------- not covered
[00:09:09] 317 | | }
[00:09:09]     | |_- `mir::interpret::error::InterpError<'tcx, O>` defined here
[00:09:09] 324 |           match *self {
[00:09:09] 324 |           match *self {
[00:09:09]     |                 ^^^^^ pattern `InvalidIntToPtrCast` not covered
[00:09:09]     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
[00:09:09] 
[00:09:26] error: aborting due to previous error
[00:09:26] 
---
travis_time:end:02f25678:start=1561133526767169279,finish=1561133526772530632,duration=5361353
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07aa68e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20ba5760
travis_time:start:20ba5760
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06142020
$ dmesg | grep -i kill
