plain
travis_time:end:07cc4010:start=1552629751535279071,finish=1552629753627371978,duration=2092092907
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:55] 
[01:19:55] running 120 tests
[01:20:21] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:20:25] .i......iii.i.....ii
[01:20:25] 
[01:20:25]  finished in 30.570
[01:20:25] travis_fold:end:test_debuginfo

---
[01:29:58]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:30:00] error[E0423]: expected function, found macro `assert_eq`
[01:30:00]    --> src/libcore/../libcore/tests/num/int_macros.rs:31:9
[01:30:00]     |
[01:30:00] 1   | / macro_rules! int_module { ($T:ident, $T_i:ident) => (
[01:30:00] 2   | | #[cfg(test)]
[01:30:00] 3   | | mod tests {
[01:30:00] 4   | |     use core::$T_i::*;
[01:30:00] ...   |
[01:30:00] 31  | |         assert_eq((0 as $T).abs(), 0 as $T);
[01:30:00]     | |         ^^^^^^^^^ help: use `!` to invoke the macro: `assert_eq!`
[01:30:00] 213 | |
[01:30:00] 214 | | )}
[01:30:00] 214 | | )}
[01:30:00]     | |__- in this expansion of `int_module!`
[01:30:00]    ::: src/libcore/../libcore/tests/num/i8.rs:1:1
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   |   int_module!(i8, i8);
[01:30:00]     |   -------------------- in this macro invocation
[01:30:00] error[E0423]: expected function, found macro `assert_eq`
[01:30:00]    --> src/libcore/../libcore/tests/num/int_macros.rs:31:9
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   | / macro_rules! int_module { ($T:ident, $T_i:ident) => (
[01:30:00] 2   | | #[cfg(test)]
[01:30:00] 3   | | mod tests {
[01:30:00] 4   | |     use core::$T_i::*;
[01:30:00] ...   |
[01:30:00] 31  | |         assert_eq((0 as $T).abs(), 0 as $T);
[01:30:00]     | |         ^^^^^^^^^ help: use `!` to invoke the macro: `assert_eq!`
[01:30:00] 213 | |
[01:30:00] 214 | | )}
[01:30:00] 214 | | )}
[01:30:00]     | |__- in this expansion of `int_module!`
[01:30:00]    ::: src/libcore/../libcore/tests/num/i16.rs:1:1
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   |   int_module!(i16, i16);
[01:30:00]     |   ---------------------- in this macro invocation
[01:30:00] error[E0423]: expected function, found macro `assert_eq`
[01:30:00]    --> src/libcore/../libcore/tests/num/int_macros.rs:31:9
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   | / macro_rules! int_module { ($T:ident, $T_i:ident) => (
[01:30:00] 2   | | #[cfg(test)]
[01:30:00] 3   | | mod tests {
[01:30:00] 4   | |     use core::$T_i::*;
[01:30:00] ...   |
[01:30:00] 31  | |         assert_eq((0 as $T).abs(), 0 as $T);
[01:30:00]     | |         ^^^^^^^^^ help: use `!` to invoke the macro: `assert_eq!`
[01:30:00] 213 | |
[01:30:00] 214 | | )}
[01:30:00] 214 | | )}
[01:30:00]     | |__- in this expansion of `int_module!`
[01:30:00]    ::: src/libcore/../libcore/tests/num/i32.rs:1:1
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   |   int_module!(i32, i32);
[01:30:00]     |   ---------------------- in this macro invocation
[01:30:00] error[E0423]: expected function, found macro `assert_eq`
[01:30:00]    --> src/libcore/../libcore/tests/num/int_macros.rs:31:9
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   | / macro_rules! int_module { ($T:ident, $T_i:ident) => (
[01:30:00] 2   | | #[cfg(test)]
[01:30:00] 3   | | mod tests {
[01:30:00] 4   | |     use core::$T_i::*;
[01:30:00] ...   |
[01:30:00] 31  | |         assert_eq((0 as $T).abs(), 0 as $T);
[01:30:00]     | |         ^^^^^^^^^ help: use `!` to invoke the macro: `assert_eq!`
[01:30:00] 213 | |
[01:30:00] 214 | | )}
[01:30:00] 214 | | )}
[01:30:00]     | |__- in this expansion of `int_module!`
[01:30:00]    ::: src/libcore/../libcore/tests/num/i64.rs:1:1
[01:30:00]     |
[01:30:00]     |
[01:30:00] 1   |   int_module!(i64, i64);
[01:30:00]     |   ---------------------- in this macro invocation
[01:30:12] error: aborting due to 4 previous errors
[01:30:12] 
[01:30:12] For more information about this error, try `rustc --explain E0423`.
[01:30:12] error: Could not compile `core`.
[01:30:12] error: Could not compile `core`.
[01:30:12] 
[01:30:12] To learn more, run the command again with --verbose.
[01:30:12] 
[01:30:12] 
[01:30:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:30:12] 
[01:30:12] 
[01:30:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:12] Build completed unsuccessfully in 0:22:05
[01:30:12] Build completed unsuccessfully in 0:22:05
[01:30:12] make: *** [check] Error 1
[01:30:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:189befc3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 15 07:32:57 UTC 2019
---
travis_time:end:01db0c6c:start=1552635178839284636,finish=1552635178844826208,duration=5541572
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ffce96
$ ln -s . chetravis_time:end:06ffce96:start=1552635178850024094,finish=1552635178920692886,duration=70668792
travis_fold:start:after_failure.5
travis_time:start:01dda9b8
travis_time:start:01dda9b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1842e8aa
$ dmesg | grep -i kill
