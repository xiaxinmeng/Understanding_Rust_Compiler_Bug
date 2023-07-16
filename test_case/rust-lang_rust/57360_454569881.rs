plain
travis_time:end:08356510:start=1547589672919180181,finish=1547589675227109024,duration=2307928843
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:18]    Compiling cc v1.0.28
[00:04:18]    Compiling libc v0.2.46
[00:04:18]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:18]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:18] error: expected one of `,` or `}`, found `.`
[00:04:18]    --> src/libcore/iter/adapters/flatten.rs:236:18
[00:04:18]     |
[00:04:18] 236 |                 f.checked_add(b)?.checked_add(m.checked_mul(max)?)?
[00:04:18]     |                  ^ expected one of `,` or `}` here
[00:04:19]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:23] error[E0574]: expected struct, variant or union type, found macro `try`
[00:04:23]    --> src/libcore/iter/adapters/flatten.rs:235:55
[00:04:23]     |
[00:04:23]     |
[00:04:23] 235 |             (Some(f), Some(b), Some(m), Some(max)) => try {
[00:04:23]     |                                                       ^^^ did you mean `try!(...)`?
[00:04:24]    Compiling compiler_builtins v0.1.4
[00:04:24]    Compiling cmake v0.1.33
[00:04:24]    Compiling backtrace-sys v0.1.27
[00:04:26]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:37] expected success, got: exit code: 101
[00:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:37] Build completed unsuccessfully in 0:00:20
[00:04:37] make: *** [all] Error 1
[00:04:37] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:301c870a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 22:06:03 UTC 2019
