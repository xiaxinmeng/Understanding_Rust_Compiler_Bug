plain
travis_time:end:2744ac60:start=1549283403910969427,finish=1549283806403010682,duration=402492041255
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:08]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:24:08]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:24:08]    Compiling rustc-demangle v0.1.10
[00:24:15]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:24:16] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:24:16]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:24:16] 82 | #[allow_internal_unstable]
[00:24:16]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:24:16] 
[00:24:35]     Finished release [optimized] target(s) in 1m 09s
---
[00:25:41]    Compiling parking_lot v0.6.4
[00:25:44]    Compiling rustc-rayon v0.1.1
[00:25:48]    Compiling tempfile v3.0.5
[00:25:48]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:25:48] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:25:48]   |
[00:25:48] 4 | #[allow_internal_unstable]
[00:25:48]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:25:48] 
[00:25:48] 
[00:25:51]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:25:52]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:25:52] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:25:52]   --> /checkout/src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:25:52] 82 | #[allow_internal_unstable]
[00:25:52]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:25:52] 
[00:25:52] 
[00:25:52] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:25:52]    |
[00:25:52] 71 | #[allow_internal_unstable]
[00:25:52]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:25:52] 
[00:25:52] 
[00:26:01]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:26:13] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:13]    |
[00:26:13] 71 | #[allow_internal_unstable]
[00:26:13]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:13] 
[00:26:13] 
[00:26:13] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:26:13]   |
[00:26:13] 4 | #[allow_internal_unstable]
[00:26:13]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:26:13] 
[00:26:13] 
[00:27:29]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:27:30] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:27:30]   |
[00:27:30] 4 | #[allow_internal_unstable]
[00:27:30]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:27:30] 
[00:27:30] 
[00:27:31] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:27:31]    |
[00:27:31] 71 | #[allow_internal_unstable]
[00:27:31]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:27:31] 
---
[00:56:49]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:56:49]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:56:49]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:56:49]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:56:50] warning: allow_internal_unstable expects list of feature names. In the future this will become a hard error. Please use `allow_internal_unstable(foo, bar)` to only allow the `foo` and `bar` features
[00:56:50]   --> src/libstd/../stdsimd/crates/std_detect/src/detect/arch/x86.rs:82:1
[00:56:50] 82 | #[allow_internal_unstable]
[00:56:50]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:56:50] 
[00:57:01]     Finished release [optimized] target(s) in 18.08s
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:51] 
[01:08:51] running 119 tests
[01:09:15] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:20] i......iii.i.....ii
[01:09:20] 
[01:09:20]  finished in 29.216
[01:09:20] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:27] 
[01:09:27] running 47 tests
[01:10:57] ....................................FFF.......test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:12:55] .
[01:12:55] failures:
[01:12:55] 
[01:12:55] ---- [run-pass] run-pass-fulldeps/plugin-args-1.rs stdout ----
[01:12:55] ---- [run-pass] run-pass-fulldeps/plugin-args-1.rs stdout ----
[01:12:55] 
[01:12:55] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:12:55] status: exit code: 1
[01:12:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/plugin-args-1/auxiliary"
[01:12:55] ------------------------------------------
[01:12:55] 
[01:12:55] ------------------------------------------
[01:12:55] stderr:
[01:12:55] stderr:
[01:12:55] ------------------------------------------
[01:12:55] {"message":"unused import: `syntax::ptr::P`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":397,"byte_end":411,"line_start":17,"line_end":17,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use syntax::ptr::P;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::ptr::P`\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:17:5\n   |\nLL | use syntax::ptr::P;\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:12:55] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n