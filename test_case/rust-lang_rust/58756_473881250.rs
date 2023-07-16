plain
travis_time:end:140483b4:start=1552910074182578083,finish=1552910075151753730,duration=969175647
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f32.rs:536:5
[00:04:42]     |
[00:04:42] 536 |     pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
[00:04:42]     |
[00:04:42]     = note: `-D missing-docs` implied by `-D warnings`
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f32.rs:543:5
[00:04:42]     |
[00:04:42] 543 |     pub const fn from_le_bytes(bytes: [u8; 4]) -> Self {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f32.rs:550:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 550 |     pub const fn from_ne_bytes(bytes: [u8; 4]) -> Self {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f64.rs:494:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 494 |     pub const fn to_be_bytes(self) -> [u8; 8] {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f64.rs:501:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 501 |     pub const fn to_le_bytes(self) -> [u8; 8] {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f64.rs:508:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 508 |     pub const fn to_ne_bytes(self) -> [u8; 8] {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f64.rs:515:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 515 |     pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f64.rs:522:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 522 |     pub const fn from_le_bytes(bytes: [u8; 8]) -> Self {
[00:04:42] 
[00:04:42] error: missing documentation for a method
[00:04:42]    --> src/libcore/num/f64.rs:529:5
[00:04:42]     |
[00:04:42]     |
[00:04:42] 529 |     pub const fn from_ne_bytes(bytes: [u8; 8]) -> Self {
[00:04:42] 
[00:04:43] error: aborting due to 9 previous errors
[00:04:43] 
[00:04:43] error: Could not compile `core`.
---
travis_time:end:02a6fb47:start=1552910369789953950,finish=1552910369795876935,duration=5922985
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:285fb487
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2abc0ac8
travis_time:start:2abc0ac8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26f79a02
$ dmesg | grep -i kill
