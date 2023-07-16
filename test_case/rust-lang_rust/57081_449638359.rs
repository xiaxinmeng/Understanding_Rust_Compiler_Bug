plain
travis_time:end:037ef9a0:start=1545573286639586429,finish=1545573288777596451,duration=2138010022
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:00:42] Successfully built f8222eb95fb0
[00:00:42] Successfully tagged rust-ci:latest
[00:00:42] Built container sha256:f8222eb95fb0491f0130df2d7ab67057d2abdc30340c317852535d32b88bd1c3
[00:00:42] Uploading finished image to s3://rust-lang-ci-sccache2/docker/edb4b0c182cd03fb6cb15ee5994e47ebf5e2bcbda67b08b896aeaed85b4803bd22dc752096b5ed6913438390dc397259347c24173ab37bafa134d42f055fc78d
[00:01:27] upload failed: - to s3://rust-lang-ci-sccache2/docker/edb4b0c182cd03fb6cb15ee5994e47ebf5e2bcbda67b08b896aeaed85b4803bd22dc752096b5ed6913438390dc397259347c24173ab37bafa134d42f055fc78d Unable to locate credentials

[00:01:27] travis_time:end:0a440dc2:start=1545573304382236159,finish=1545573384705961904,duration=80323725745
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:01:27] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---
[00:05:29]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:30]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:34]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:47]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:15] error[E0599]: no method named `unwrap` found for type `&ty::query::plumbing::QueryValue<std::result::Result<&ty::sty::Const<'_>, mir::interpret::error::ErrorHandled>>` in the current scope
[00:07:15]    --> src/librustc/ty/query/on_disk_cache.rs:240:48
[00:07:15]     |
[00:07:15] 240 |                     let (value, index) = entry.unwrap();
[00:07:15] 
[00:07:15] 
[00:07:15] error[E0599]: no method named `unwrap` found for type `&ty::query::plumbing::QueryValue<<Q as ty::query::config::QueryConfig<'_>>::Value>` in the current scope
[00:07:15]     --> src/librustc/ty/query/on_disk_cache.rs:1099:36
[00:07:15]      |
[00:07:15] 1099 |         let (value, index) = entry.unwrap();
[00:07:15] 
[00:07:17] error: aborting due to 2 previous errors
[00:07:17] 
[00:07:17] For more information about this error, try `rustc --explain E0599`.
[00:07:17] For more information about this error, try `rustc --explain E0599`.
[00:07:17] error: Could not compile `rustc`.
[00:07:17] 
[00:07:17] To learn more, run the command again with --verbose.
[00:07:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:17] expected success, got: exit code: 101
[00:07:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:17] Build completed unsuccessfully in 0:03:46
[00:07:17] Makefile:28: recipe for target 'all' failed
[00:07:17] make: *** [all] Error 1
20556 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
19936 ./src/tools/lldb/source
19256 ./src/tools/lldb/www/cpp_reference
19252 ./src/tools/lldb/www/cpp_reference/html
---
travis_time:end:17c15830:start=1545573735814996397,finish=1545573735819467464,duration=4471067
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05df3890
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:017f5fc5
travis_time:start:017f5fc5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
