plain
travis_time:end:069f5dcf:start=1546535304381827813,finish=1546535305520713361,duration=1138885548
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:21:38] 
[01:21:38] ------------------------------------------
[01:21:38] stderr:
[01:21:38] ------------------------------------------
[01:21:38] {"message":"src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])\n\n"}
[01:21:38] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
[01:21:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:38] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:21:38] note: the compiler unexpectedly panicked. this is a bug.
[01:21:38] 
[01:21:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:21:38] 
---
[01:29:47]     |
[01:29:47] 151 |                 .position(|line| line.trim_right().ends_with("*/"))
[01:29:47]     |                                       ^^^^^^^^^^
[01:29:47] 
[01:31:20] warning[E0597]: `msg.method` does not live long enough
[01:31:20]     |
[01:31:20] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:31:20] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:31:20]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:31:20] 260 |             msg.method;
[01:31:20] 260 |             msg.method;
[01:31:20]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:31:20] 290 |     }
[01:31:20] 290 |     }
[01:31:20]     |     - `msg.method` dropped here while still borrowed
[01:31:20]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:31:20]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:31:20] 
[01:33:33]     Finished release [optimized] target(s) in 11m 55s
[01:33:33]     Finished release [optimized] target(s) in 11m 55s
[01:33:33] travis_fold:end:stage2-rls

[01:33:33] travis_time:end:stage2-rls:start=1546540211966991711,finish=1546540927693574348,duration=715726582637

[01:33:34]    Compiling difference v2.0.0
[01:33:35]    Compiling rls v1.31.6 (/checkout/src/tools/rls)
[01:33:46] warning[E0597]: `msg.method` does not live long enough
[01:33:46]     |
[01:33:46] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:33:46] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:33:46]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:33:46] 260 |             msg.method;
[01:33:46] 260 |             msg.method;
[01:33:46]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:33:46] 290 |     }
[01:33:46] 290 |     }
[01:33:46]     |     - `msg.method` dropped here while still borrowed
[01:33:46]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:33:46]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:33:46] 
[01:36:06]     Finished release [optimized] target(s) in 2m 32s
---
[01:44:45] Verifying status of rustfmt...
[01:44:45] Verifying status of clippy-driver...
[01:44:45] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:44:45] 
[01:44:45] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:44:45] 
[01:44:45] If you do intend to update 'clippy-driver', please check the error messages above and
[01:44:45] commit another update.
[01:44:45] 
[01:44:45] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:44:45] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:44:45] proper steps.
4482940 ./obj
4482900 ./obj/build
3823040 ./obj/build/x86_64-unknown-linux-gnu
1337124 ./obj/build/x86_64-unknown-linux-gnu/llvm
