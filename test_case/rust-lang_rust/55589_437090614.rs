plain
travis_time:end:212082ae:start=1541698013725436836,finish=1541698016154653707,duration=2429216871
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:06:57]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:07:06]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:25]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:12:25]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:25] error: expected one of `)` or `,`, found `.`
[00:12:25]    --> librustc_mir/interpret/validity.rs:216:14
[00:12:25] 216 |         &self.ecx,
[00:12:25] 216 |         &self.ecx,
[00:12:25]     |              ^ expected one of `)` or `,` here
[00:12:27] error[E0425]: cannot find value `field` in this scope
[00:12:27]    --> librustc_mir/interpret/validity.rs:262:33
[00:12:27]     |
[00:12:27]     |
[00:12:27] 262 |         self.visit_elem(old_op, field, new_op, PathElem::Variant(name))
[00:12:27] 
[00:12:27] 
[00:12:39] error[E0599]: no method named `visit_elem` found for type `&mut interpret::validity::ValidityVisitor<'rt, 'a, 'mir, 'tcx, M>` in the current scope
[00:12:39]    --> librustc_mir/interpret/validity.rs:251:14
[00:12:39]     |
[00:12:39] 251 |         self.visit_elem(old_op, field, new_op, elem)
[00:12:39] 
[00:12:39] 
[00:12:39] error[E0599]: no method named `visit_elem` found for type `&mut interpret::validity::ValidityVisitor<'rt, 'a, 'mir, 'tcx, M>` in the current scope
[00:12:39]    --> librustc_mir/interpret/validity.rs:262:14
[00:12:39]     |
[00:12:39] 262 |         self.visit_elem(old_op, field, new_op, PathElem::Variant(name))
[00:12:39] 
[00:12:41] error: aborting due to 4 previous errors
[00:12:41] 
[00:12:41] Some errors occurred: E0425, E0599.
[00:12:41] Some errors occurred: E0425, E0599.
[00:12:41] For more information about an error, try `rustc --explain E0425`.
[00:12:41] error: Could not compile `rustc_mir`.
[00:12:41] warning: build failed, waiting for other jobs to finish...
[00:13:23] error: build failed
[00:13:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:23] expected success, got: exit code: 101
[00:13:23] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:13:23] travis_fold:end:stage0-rustc

[00:13:23] travis_time:end:stage0-rustc:start=1541698320979205089,finish=1541698829739286414,duration=508760081325


[00:13:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:23] Build completed unsuccessfully in 0:09:33
[00:13:23] make: *** [all] Error 1
[00:13:23] Makefile:28: recipe for target 'all' failed
439124 ./.git/modules/src/tools/clang/objects
439116 ./.git/modules/src/tools/clang/objects/pack
373240 ./src/llvm
363748 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib
---
151412 ./src/tools/clang
150256 ./obj/build/bootstrap/debug/incremental
149124 ./src/llvm-emscripten/test
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6hid1bxeu-12l6qn7-22tmsi8iacpi9
111092 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
93748 ./src/tools/clang/test
---
travis_time:end:13c7ba84:start=1541698830418063535,finish=1541698830422747133,duration=4683598
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07f2d236
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e72146
travis_time:start:01e72146
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09920a40
$ dmesg | grep -i kill
