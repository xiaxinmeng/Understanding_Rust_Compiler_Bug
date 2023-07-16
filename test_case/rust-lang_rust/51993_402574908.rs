plain
[00:23:25]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:23:29]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:24:33]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:24:45]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:26:33] error[E0597]: borrowed value does not live long enough
[00:26:33]     --> librustc/ty/context.rs:1767:37
[00:26:33]      |
[00:26:33] 1767 |         let _reset = OnDrop(move || TLV.with(|tlv| tlv.set(old)));
[00:26:33]      |                                     ^^^                        - temporary value only lives until here
[00:26:33]      |                                     |
[00:26:33]      |                                     temporary value does not live long enough
[00:26:33]      |
[00:26:33]      = note: borrowed value must be valid for the static lifetime...
[00:26:33] 
[00:26:33] error[E0597]: borrowed value does not live long enough
[00:26:33]     --> librustc/ty/context.rs:1811:13
[00:26:33]      |
[00:26:33] 1811 |             TRACK_DIAGNOSTICS.with(|current| {
[00:26:33]      |             ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:26:33] 1821 |         })
[00:26:33] 1821 |         })
[00:26:33]      |         - temporary value only lives until here
[00:26:33]      |
[00:26:33]      = note: borrowed value must be valid for the static lifetime...
elease/build/backtrace-sys-36987d8923861156/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-b497c8cb5a7907ba/out` (exit code: 101)
[00:26:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:26:35] expected success, got: exit code: 101
[00:26:35] expected success, got: exit code: 101
[00:26:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:26:35] travis_fold:end:stage1-rustc

[00:26:35] travis_time:end:stage1-rustc:start=1530749450076749667,finish=1530749684221167782,duration=234144418115


[00:26:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:35] Build completed unsuccessfully in 0:21:43
[00:26:35] make: *** [all] Error 1
[00:26:35] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06cf43d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
