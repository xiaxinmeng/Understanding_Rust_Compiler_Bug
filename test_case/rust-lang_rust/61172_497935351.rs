plain
[01:29:44]    Compiling cargo v0.38.0 (/checkout/src/tools/cargo)
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]   --> src/tools/cargo/src/cargo/core/compiler/build_context/mod.rs:19:33
[01:30:00]    |
[01:30:00] 19 | pub struct BuildContext<'a, 'cfg: 'a> {
[01:30:00]    |
[01:30:00] note: lint level defined here
[01:30:00]   --> src/tools/cargo/src/cargo/lib.rs:3:9
[01:30:00]    |
[01:30:00]    |
[01:30:00] 3  | #![warn(rust_2018_idioms)]
[01:30:00]    |         ^^^^^^^^^^^^^^^^
[01:30:00]    = note: #[warn(explicit_outlives_requirements)] implied by #[warn(rust_2018_idioms)]
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]   --> src/tools/cargo/src/cargo/core/compiler/context/mod.rs:30:28
[01:30:00]    |
[01:30:00]    |
[01:30:00] 30 | pub struct Context<'a, 'cfg: 'a> {
[01:30:00] 
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]   --> src/tools/cargo/src/cargo/core/compiler/context/unit_dependencies.rs:28:16
[01:30:00]    |
[01:30:00]    |
[01:30:00] 28 | struct State<'a: 'tmp, 'cfg: 'a, 'tmp> {
[01:30:00] help: remove these bounds
[01:30:00]    |
[01:30:00]    |
[01:30:00] 28 | struct State<'a, 'cfg, 'tmp> {
[01:30:00] 
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]   --> src/tools/cargo/src/cargo/core/compiler/context/compilation_files.rs:53:37
[01:30:00]    |
[01:30:00]    |
[01:30:00] 53 | pub struct CompilationFiles<'a, 'cfg: 'a> {
[01:30:00] 
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]    --> src/tools/cargo/src/cargo/core/package.rs:293:30
[01:30:00]     |
[01:30:00]     |
[01:30:00] 293 | pub struct Downloads<'a, 'cfg: 'a> {
[01:30:00] 
[01:30:00] warning: outlives requirements can be inferred
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]    --> src/tools/cargo/src/cargo/core/resolver/encode.rs:328:37
[01:30:00]     |
[01:30:00] 328 | pub struct WorkspaceResolve<'a, 'cfg: 'a> {
[01:30:00] 
[01:30:00] warning: outlives requirements can be inferred
[01:30:00]    --> src/tools/cargo/src/cargo/core/workspace.rs:128:28
[01:30:00]     |
[01:30:00]     |
[01:30:00] 128 | pub struct Members<'a, 'cfg: 'a> {
[01:30:00] 
[01:33:39] [RUSTC-TIMING] cargo test:false 235.634
[01:34:07] [RUSTC-TIMING] cargo test:false 28.168
[01:34:07]     Finished release [optimized] target(s) in 10m 39s
---
[01:38:14]    Compiling cargo v0.38.0 (/checkout/src/tools/cargo)
[01:38:31] error: outlives requirements can be inferred
[01:38:31]   --> src/tools/cargo/src/cargo/core/compiler/build_context/mod.rs:19:33
[01:38:31]    |
[01:38:31] 19 | pub struct BuildContext<'a, 'cfg: 'a> {
[01:38:31]    |
[01:38:31] note: lint level defined here
[01:38:31]   --> src/tools/cargo/src/cargo/lib.rs:1:24
[01:38:31]    |
[01:38:31]    |
[01:38:31] 1  | #![cfg_attr(test, deny(warnings))]
[01:38:31]    |                        ^^^^^^^^
[01:38:31]    = note: #[deny(explicit_outlives_requirements)] implied by #[deny(warnings)]
[01:38:31] 
[01:38:31] error: outlives requirements can be inferred
[01:38:31]   --> src/tools/cargo/src/cargo/core/compiler/context/mod.rs:30:28
[01:38:31]    |
[01:38:31] 30 | pub struct Context<'a, 'cfg: 'a> {
[01:38:31] 
[01:38:31] error: outlives requirements can be inferred
[01:38:31]   --> src/tools/cargo/src/cargo/core/compiler/context/unit_dependencies.rs:28:16
[01:38:31]    |
[01:38:31]    |
[01:38:31] 28 | struct State<'a: 'tmp, 'cfg: 'a, 'tmp> {
[01:38:31] help: remove these bounds
[01:38:31]    |
[01:38:31]    |
[01:38:31] 28 | struct State<'a, 'cfg, 'tmp> {
[01:38:31] 
[01:38:31] error: outlives requirements can be inferred
[01:38:31]   --> src/tools/cargo/src/cargo/core/compiler/context/compilation_files.rs:53:37
[01:38:31]    |
[01:38:31]    |
[01:38:31] 53 | pub struct CompilationFiles<'a, 'cfg: 'a> {
[01:38:31] 
[01:38:31] error: outlives requirements can be inferred
[01:38:31]    --> src/tools/cargo/src/cargo/core/package.rs:293:30
[01:38:31]     |
[01:38:31]     |
[01:38:31] 293 | pub struct Downloads<'a, 'cfg: 'a> {
[01:38:31] 
[01:38:31] error: outlives requirements can be inferred
[01:38:31] error: outlives requirements can be inferred
[01:38:31]    --> src/tools/cargo/src/cargo/core/resolver/encode.rs:328:37
[01:38:31]     |
[01:38:31] 328 | pub struct WorkspaceResolve<'a, 'cfg: 'a> {
[01:38:31] 
[01:38:31] error: outlives requirements can be inferred
[01:38:31]    --> src/tools/cargo/src/cargo/core/workspace.rs:128:28
[01:38:31]     |
[01:38:31]     |
[01:38:31] 128 | pub struct Members<'a, 'cfg: 'a> {
[01:38:31] 
[01:38:31] error: aborting due to 7 previous errors
[01:38:31] 
[01:38:32] [RUSTC-TIMING] cargo test:true 17.842
---
[01:39:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:39:48] expected success, got: exit code: 101
[01:39:48] 
[01:39:48] 
[01:39:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:39:48] Build completed unsuccessfully in 1:36:19
[01:39:48] make: *** [check-aux] Error 1
[01:39:48] Makefile:50: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1284ab68
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jun  1 11:09:05 UTC 2019
---
travis_time:end:2ea3428e:start=1559387346501309046,finish=1559387346515869624,duration=14560578
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b855920
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:145f8b0f
travis_time:start:145f8b0f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00fe0b8c
$ dmesg | grep -i kill
