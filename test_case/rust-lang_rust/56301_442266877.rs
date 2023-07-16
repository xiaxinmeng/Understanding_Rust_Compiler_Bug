plain
travis_time:end:18769107:start=1543361681520406826,finish=1543361682652815024,duration=1132408198
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:23:33]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:23:42]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:30:28]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:30:28]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:31:40] error[E0502]: cannot borrow `block_data.statements` as mutable because it is also borrowed as immutable
[00:31:40]    --> src/librustc_mir/transform/add_retag.rs:185:29
[00:31:40]     |
[00:31:40] 167 |                 match block_data.statements[i].kind {
[00:31:40] ...
[00:31:40] ...
[00:31:40] 185 |                             block_data.statements.insert(i, Statement {
[00:31:40] 186 |                                 source_info,
[00:31:40] 186 |                                 source_info,
[00:31:40] 187 |                                 kind: StatementKind::EscapeToRaw(src.clone()),
[00:31:40]     |                                                                  --- immutable borrow used here, in later iteration of loop
[00:31:40] 
[00:31:40] error[E0502]: cannot borrow `block_data.statements` as mutable because it is also borrowed as immutable
[00:31:40]    --> src/librustc_mir/transform/add_retag.rs:197:25
[00:31:40]     |
[00:31:40] 167 |                 match block_data.statements[i].kind {
[00:31:40] ...
[00:31:40] ...
[00:31:40] 197 |                         block_data.statements.insert(i+1, Statement {
[00:31:40] 198 |                             source_info,
[00:31:40] 198 |                             source_info,
[00:31:40] 199 |                             kind: StatementKind::Retag { fn_entry: false, place: place.clone() },
[00:31:40]     |                                                                                  ----- immutable borrow used here, in later iteration of loop
[00:31:48] error: aborting due to 2 previous errors
[00:31:48] 
[00:31:48] For more information about this error, try `rustc --explain E0502`.
[00:31:48] error: Could not compile `rustc_mir`.
[00:31:48] error: Could not compile `rustc_mir`.
[00:31:48] warning: build failed, waiting for other jobs to finish...
[00:32:46] error: build failed
[00:32:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:32:46] expected success, got: exit code: 101
[00:32:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:32:46] Build completed unsuccessfully in 0:29:23
[00:32:46] Makefile:28: recipe for target 'all' failed
[00:32:46] make: *** [all] Error 1
35580 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
35572 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
34948 ./src/llvm/test/tools
33444 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
