plain
travis_time:end:004c4a00:start=1560608347399422631,finish=1560608349567199645,duration=2167777014
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:46]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:49] error[E0432]: unresolved import `measureme`
[00:08:49]   --> src/librustc/util/profiling.rs:12:5
[00:08:49]    |
[00:08:49] 12 | use measureme::{StringId, TimestampKind};
[00:08:49]    |     ^^^^^^^^^ use of undeclared type or module `measureme`
[00:08:50] error[E0433]: failed to resolve: use of undeclared type or module `measureme`
[00:08:50]   --> src/librustc/util/profiling.rs:17:17
[00:08:50]    |
[00:08:50]    |
[00:08:50] 17 | type Profiler = measureme::Profiler<measureme::MmapSerializationSink>;
[00:08:50]    |                 ^^^^^^^^^ use of undeclared type or module `measureme`
[00:08:50] error[E0433]: failed to resolve: use of undeclared type or module `measureme`
[00:08:50]   --> src/librustc/util/profiling.rs:17:37
[00:08:50]    |
[00:08:50]    |
[00:08:50] 17 | type Profiler = measureme::Profiler<measureme::MmapSerializationSink>;
[00:08:50]    |                                     ^^^^^^^^^ use of undeclared type or module `measureme`
[00:09:18] error: aborting due to 3 previous errors
[00:09:18] 
[00:09:18] Some errors have detailed explanations: E0432, E0433.
[00:09:18] For more information about an error, try `rustc --explain E0432`.
---
travis_time:end:0623c3b0:start=1560608919957818426,finish=1560608919962439389,duration=4620963
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01ef04b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b6d7080
travis_time:start:0b6d7080
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22d4c9b4
$ dmesg | grep -i kill
