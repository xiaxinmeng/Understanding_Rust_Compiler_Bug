plain
travis_time:end:1570503c:start=1551196401450103822,finish=1551196405862117709,duration=4412013887
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:09:52]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:09:55] error[E0308]: mismatched types
[00:09:55]    --> src/libcore/num/f64.rs:516:20
[00:09:55]     |
[00:09:55] 516 |         swap_if_le(Self::from_ne_bytes(bytes))
[00:09:55]     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected array of 8 elements, found f64
[00:09:55]     |
[00:09:55]     = note: expected type `[u8; 8]`
[00:09:55]                found type `f64`
[00:09:55] error[E0308]: mismatched types
[00:09:55]    --> src/libcore/num/f64.rs:516:9
[00:09:55]     |
[00:09:55]     |
[00:09:55] 515 |     pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
[00:09:55]     |                                                   ---- expected `f64` because of return type
[00:09:55] 516 |         swap_if_le(Self::from_ne_bytes(bytes))
[00:09:55]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected f64, found array of 8 elements
[00:09:55]     = note: expected type `f64`
[00:09:55]     = note: expected type `f64`
[00:09:55]                found type `[u8; 8]`
[00:09:55] error[E0308]: mismatched types
[00:09:55]    --> src/libcore/num/f64.rs:523:20
[00:09:55]     |
[00:09:55]     |
[00:09:55] 523 |         swap_if_be(Self::from_ne_bytes(bytes))
[00:09:55]     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected array of 8 elements, found f64
[00:09:55]     |
[00:09:55]     = note: expected type `[u8; 8]`
[00:09:55]                found type `f64`
[00:09:55] error[E0308]: mismatched types
[00:09:55]    --> src/libcore/num/f64.rs:523:9
[00:09:55]     |
[00:09:55]     |
[00:09:55] 522 |     pub const fn from_le_bytes(bytes: [u8; 8]) -> Self {
[00:09:55]     |                                                   ---- expected `f64` because of return type
[00:09:55] 523 |         swap_if_be(Self::from_ne_bytes(bytes))
[00:09:55]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected f64, found array of 8 elements
[00:09:55]     = note: expected type `f64`
[00:09:55]     = note: expected type `f64`
[00:09:55]                found type `[u8; 8]`
[00:10:01] error: aborting due to 4 previous errors
[00:10:01] 
[00:10:01] For more information about this error, try `rustc --explain E0308`.
[00:10:01] error: Could not compile `core`.
---
travis_time:end:046ffba8:start=1551197019631286950,finish=1551197019636406152,duration=5119202
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00982b0c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a3e86b8
travis_time:start:0a3e86b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e7167ce
$ dmesg | grep -i kill
