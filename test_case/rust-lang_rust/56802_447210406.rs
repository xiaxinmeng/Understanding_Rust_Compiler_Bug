plain
travis_time:end:097988b8:start=1544761815736919259,finish=1544761816720112919,duration=983193660
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:25]    Compiling cc v1.0.25
[00:03:25]    Compiling libc v0.2.45
[00:03:25]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:25]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:25] error: incorrect close delimiter: `]`
[00:03:25]     |
[00:03:25]     |
[00:03:25] 472 |     #[unstable(feature = "iter_nth_back", issue = "54054"]
[00:03:25]     |      -        - un-closed delimiter                      ^ incorrect close delimiter
[00:03:25]     |      |
[00:03:25]     |      close delimiter possibly meant for this
[00:03:28]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:32]    Compiling compiler_builtins v0.1.2
[00:03:32]    Compiling cmake v0.1.33
[00:03:32]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:32]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:35]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:35]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:36]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:36]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:41]    --> src/libcore/iter/traits.rs:673:18
[00:03:41]     |
[00:03:41] 673 |         (**self).rfind(predicate)
[00:03:41]     |
[00:03:41]     = help: the trait `marker::Sized` is not implemented for `I`
[00:03:41]     = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:03:41]     = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:03:41]     = help: consider adding a `where I: marker::Sized` bound
eGen/X86
40496 ./src/llvm-emscripten/lib/Target
37776 ./src/tools/lldb/www
37208 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
---
travis_time:end:027e6838:start=1544762051466147273,finish=1544762051471372398,duration=5225125
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e960bf0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07229744
travis_time:start:07229744
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07eb5eb1
$ dmesg | grep -i kill
