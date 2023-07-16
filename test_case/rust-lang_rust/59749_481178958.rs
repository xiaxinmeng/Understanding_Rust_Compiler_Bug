plain
travis_time:end:050a5d02:start=1554802476994440304,finish=1554802479420246266,duration=2425805962
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:34]    Compiling serde_derive v1.0.81
[00:02:35]    Compiling toml v0.4.10
[00:02:35]    Compiling serde_json v1.0.33
[00:02:44]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:50] error: function is never used: `invoke_rustdoc`
[00:02:50]     |
[00:02:50]     |
[00:02:50] 324 | / fn invoke_rustdoc(
[00:02:50] 325 | |     builder: &Builder<'_>,
[00:02:50] 327 | |     target: Interned<String>,
[00:02:50] ...   |
[00:02:50] ...   |
[00:02:50] 350 | |     builder.run(&mut cmd);
[00:02:50]     | |_^
[00:02:50]     |
[00:02:50] note: lint level defined here
[00:02:50]    --> src/bootstrap/lib.rs:107:9
[00:02:50]    --> src/bootstrap/lib.rs:107:9
[00:02:50]     |
[00:02:50] 107 | #![deny(warnings)]
[00:02:50]     |         ^^^^^^^^
[00:02:50]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:02:50] error: aborting due to previous error
[00:02:50] 
[00:02:50] error: Could not compile `bootstrap`.
[00:02:50] 
[00:02:50] 
[00:02:50] To learn more, run the command again with --verbose.
[00:02:50] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:50] Build completed unsuccessfully in 0:00:58
[00:02:50] make: *** [prepare] Error 1
[00:02:50] Makefile:69: recipe for target 'prepare' failed
[00:02:51] Command failed. Attempt 2/5:
[00:02:52]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:59] error: function is never used: `invoke_rustdoc`
[00:02:59]     |
[00:02:59]     |
[00:02:59] 324 | / fn invoke_rustdoc(
[00:02:59] 325 | |     builder: &Builder<'_>,
[00:02:59] 327 | |     target: Interned<String>,
[00:02:59] ...   |
[00:02:59] ...   |
[00:02:59] 350 | |     builder.run(&mut cmd);
[00:02:59]     | |_^
[00:02:59]     |
[00:02:59] note: lint level defined here
[00:02:59]    --> src/bootstrap/lib.rs:107:9
[00:02:59]    --> src/bootstrap/lib.rs:107:9
[00:02:59]     |
[00:02:59] 107 | #![deny(warnings)]
[00:02:59]     |         ^^^^^^^^
[00:02:59]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:02:59] error: aborting due to previous error
[00:02:59] 
[00:02:59] error: Could not compile `bootstrap`.
[00:02:59] 
[00:02:59] 
[00:02:59] To learn more, run the command again with --verbose.
[00:02:59] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:59] Build completed unsuccessfully in 0:00:07
[00:02:59] make: *** [prepare] Error 1
[00:02:59] Makefile:69: recipe for target 'prepare' failed
[00:03:01] Command failed. Attempt 3/5:
[00:03:01]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:08] error: function is never used: `invoke_rustdoc`
[00:03:08]     |
[00:03:08]     |
[00:03:08] 324 | / fn invoke_rustdoc(
[00:03:08] 325 | |     builder: &Builder<'_>,
[00:03:08] 327 | |     target: Interned<String>,
[00:03:08] ...   |
[00:03:08] ...   |
[00:03:08] 350 | |     builder.run(&mut cmd);
[00:03:08]     | |_^
[00:03:08]     |
[00:03:08] note: lint level defined here
[00:03:08]    --> src/bootstrap/lib.rs:107:9
[00:03:08]    --> src/bootstrap/lib.rs:107:9
[00:03:08]     |
[00:03:08] 107 | #![deny(warnings)]
[00:03:08]     |         ^^^^^^^^
[00:03:08]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:03:08] error: aborting due to previous error
[00:03:08] 
[00:03:08] error: Could not compile `bootstrap`.
[00:03:08] 
[00:03:08] 
[00:03:08] To learn more, run the command again with --verbose.
[00:03:08] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:08] Build completed unsuccessfully in 0:00:07
[00:03:08] make: *** [prepare] Error 1
[00:03:08] Makefile:69: recipe for target 'prepare' failed
[00:03:11] Command failed. Attempt 4/5:
[00:03:12]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:19] error: function is never used: `invoke_rustdoc`
[00:03:19]     |
[00:03:19]     |
[00:03:19] 324 | / fn invoke_rustdoc(
[00:03:19] 325 | |     builder: &Builder<'_>,
[00:03:19] 327 | |     target: Interned<String>,
[00:03:19] ...   |
[00:03:19] ...   |
[00:03:19] 350 | |     builder.run(&mut cmd);
[00:03:19]     | |_^
[00:03:19]     |
[00:03:19] note: lint level defined here
[00:03:19]    --> src/bootstrap/lib.rs:107:9
[00:03:19]    --> src/bootstrap/lib.rs:107:9
[00:03:19]     |
[00:03:19] 107 | #![deny(warnings)]
[00:03:19]     |         ^^^^^^^^
[00:03:19]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:03:19] error: aborting due to previous error
[00:03:19] 
[00:03:19] error: Could not compile `bootstrap`.
[00:03:19] 
[00:03:19] 
[00:03:19] To learn more, run the command again with --verbose.
[00:03:19] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:19] Build completed unsuccessfully in 0:00:07
[00:03:19] Makefile:69: recipe for target 'prepare' failed
[00:03:19] make: *** [prepare] Error 1
[00:03:23] Command failed. Attempt 5/5:
[00:03:24]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:30] error: function is never used: `invoke_rustdoc`
[00:03:30]     |
[00:03:30]     |
[00:03:30] 324 | / fn invoke_rustdoc(
[00:03:30] 325 | |     builder: &Builder<'_>,
[00:03:30] 327 | |     target: Interned<String>,
[00:03:30] ...   |
[00:03:30] ...   |
[00:03:30] 350 | |     builder.run(&mut cmd);
[00:03:30]     | |_^
[00:03:30]     |
[00:03:30] note: lint level defined here
[00:03:30]    --> src/bootstrap/lib.rs:107:9
[00:03:30]    --> src/bootstrap/lib.rs:107:9
[00:03:30]     |
[00:03:30] 107 | #![deny(warnings)]
[00:03:30]     |         ^^^^^^^^
[00:03:30]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
travis_time:end:19fdb1b0:start=1554802489785721564,finish=1554802701287257352,duration=211501535788
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:01cbe7a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1ccd43f9:start=1554802701998516792,finish=1554802702005251913,duration=6735121
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:092734e6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03541ee4
travis_time:start:03541ee4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02b04760
$ dmesg | grep -i kill
