plain
travis_time:end:09b3d33c:start=1547001510636797919,finish=1547001580405810453,duration=69769012534
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:25]    Compiling cc v1.0.25
[00:03:25]    Compiling libc v0.2.46
[00:03:25]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:25]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:25] error: expected one of `,` or `}`, found `.`
[00:03:25]    --> src/libcore/iter/adapters/flatten.rs:236:18
[00:03:25]     |
[00:03:25] 236 |                 f.checked_add(b)?.checked_add(m.checked_mul(max)?)?
[00:03:25]     |                  ^ expected one of `,` or `}` here
[00:03:26]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:31] error[E0422]: cannot find struct, variant or union type `catch` in this scope
[00:03:31]    --> src/libcore/iter/adapters/flatten.rs:235:55
[00:03:31]     |
[00:03:31]     |
[00:03:31] 235 |             (Some(f), Some(b), Some(m), Some(max)) => catch {
[00:03:31] 
[00:03:31]    Compiling compiler_builtins v0.1.4
[00:03:31]    Compiling cmake v0.1.33
[00:03:31]    Compiling backtrace-sys v0.1.27
---
[00:03:36]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:41] error[E0308]: mismatched types
[00:03:41]     --> src/libcore/option.rs:1178:43
[00:03:41]      |
[00:03:41] 1178 |     fn max_size_hint() -> Option<usize> { 1 }
[00:03:41]      |                           -------------   ^ expected enum `option::Option`, found integral variable
[00:03:41]      |                           |
[00:03:41]      |                           expected `option::Option<usize>` because of return type
[00:03:41]      |
[00:03:41]      = note: expected type `option::Option<usize>`
[00:03:41] 
[00:03:41] error[E0308]: mismatched types
[00:03:41]     --> src/libcore/option.rs:1226:43
[00:03:41]      |
[00:03:41]      |
[00:03:41] 1226 |     fn max_size_hint() -> Option<usize> { 1 }
[00:03:41]      |                           -------------   ^ expected enum `option::Option`, found integral variable
[00:03:41]      |                           |
[00:03:41]      |                           expected `option::Option<usize>` because of return type
[00:03:41]      |
[00:03:41]      = note: expected type `option::Option<usize>`
[00:03:41] 
[00:03:41] error[E0308]: mismatched types
[00:03:41]     --> src/libcore/option.rs:1265:43
[00:03:41]      |
[00:03:41]      |
[00:03:41] 1265 |     fn max_size_hint() -> Option<usize> { 1 }
[00:03:41]      |                           -------------   ^ expected enum `option::Option`, found integral variable
[00:03:41]      |                           |
[00:03:41]      |                           expected `option::Option<usize>` because of return type
[00:03:41]      |
[00:03:41]      = note: expected type `option::Option<usize>`
[00:03:41] 
[00:03:42] error[E0308]: mismatched types
[00:03:42]     --> src/libcore/result.rs:1083:43
[00:03:42]      |
[00:03:42]      |
[00:03:42] 1083 |     fn max_size_hint() -> Option<usize> { 1 }
[00:03:42]      |                           -------------   ^ expected enum `option::Option`, found integral variable
[00:03:42]      |                           |
[00:03:42]      |                           expected `option::Option<usize>` because of return type
[00:03:42]      |
[00:03:42]      = note: expected type `option::Option<usize>`
[00:03:42] 
[00:03:42] error[E0308]: mismatched types
[00:03:42]     --> src/libcore/result.rs:1130:43
[00:03:42]      |
[00:03:42]      |
[00:03:42] 1130 |     fn max_size_hint() -> Option<usize> { 1 }
[00:03:42]      |                           -------------   ^ expected enum `option::Option`, found integral variable
[00:03:42]      |                           |
[00:03:42]      |                           expected `option::Option<usize>` because of return type
[00:03:42]      |
[00:03:42]      = note: expected type `option::Option<usize>`
[00:03:42] 
[00:03:42] error[E0308]: mismatched types
[00:03:42]     --> src/libcore/result.rs:1175:43
[00:03:42]      |
[00:03:42]      |
[00:03:42] 1175 |     fn max_size_hint() -> Option<usize> { 1 }
[00:03:42]      |                           -------------   ^ expected enum `option::Option`, found integral variable
[00:03:42]      |                           |
[00:03:42]      |                           expected `option::Option<usize>` because of return type
[00:03:42]      |
[00:03:42]      = note: expected type `option::Option<usize>`
[00:03:42] 
[00:03:44] error: aborting due to 8 previous errors
[00:03:44] 
[00:03:44] Some errors occurred: E0308, E0422.
---
[00:03:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:44] expected success, got: exit code: 101
[00:03:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:44] Build completed unsuccessfully in 0:00:21
[00:03:44] make: *** [all] Error 1
[00:03:44] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07ab2fea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan  9 02:43:34 UTC 2019
---
travis_time:end:1a5b739e:start=1547001815085521184,finish=1547001815090210210,duration=4689026
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11504506
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0180b712
travis_time:start:0180b712
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a765984
$ dmesg | grep -i kill
