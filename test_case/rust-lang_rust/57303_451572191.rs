plain
travis_time:end:00168930:start=1546630141282873266,finish=1546630143465466141,duration=2182592875
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:28:20] 
[01:28:20] ------------------------------------------
[01:28:20] stderr:
[01:28:20] ------------------------------------------
[01:28:20] {"message":"src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=44) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=44) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])\n\n"}
[01:28:20] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
[01:28:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:28:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:28:20] note: the compiler unexpectedly panicked. this is a bug.
[01:28:20] 
[01:28:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:28:20] 
---
[01:36:56]     |
[01:36:56] 151 |                 .position(|line| line.trim_right().ends_with("*/"))
[01:36:56]     |                                       ^^^^^^^^^^
[01:36:56] 
[01:38:34] warning[E0597]: `msg.method` does not live long enough
[01:38:34]     |
[01:38:34] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:38:34] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:38:34]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:38:34] 260 |             msg.method;
[01:38:34] 260 |             msg.method;
[01:38:34]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:38:34] 290 |     }
[01:38:34] 290 |     }
[01:38:34]     |     - `msg.method` dropped here while still borrowed
[01:38:34]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:38:34]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:38:34] 
[01:40:57]     Finished release [optimized] target(s) in 12m 36s
[01:40:57]     Finished release [optimized] target(s) in 12m 36s
[01:40:57] travis_fold:end:stage2-rls

[01:40:57] travis_time:end:stage2-rls:start=1546635457961376501,finish=1546636214126521007,duration=756165144506

[01:40:57]    Compiling difference v2.0.0
[01:40:58]    Compiling rls v1.31.6 (/checkout/src/tools/rls)
[01:41:09] warning[E0597]: `msg.method` does not live long enough
[01:41:09]     |
[01:41:09] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:41:09] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:41:09]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:41:09] 260 |             msg.method;
[01:41:09] 260 |             msg.method;
[01:41:09]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:41:09] 290 |     }
[01:41:09] 290 |     }
[01:41:09]     |     - `msg.method` dropped here while still borrowed
[01:41:09]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:41:09]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:41:09] 
[01:43:35]     Finished release [optimized] target(s) in 2m 38s
---
[01:52:46] Verifying status of rustfmt...
[01:52:46] Verifying status of clippy-driver...
[01:52:46] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:52:46] 
[01:52:46] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:52:46] 
[01:52:46] If you do intend to update 'clippy-driver', please check the error messages above and
[01:52:46] commit another update.
[01:52:46] 
[01:52:46] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:52:46] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:52:46] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0e25505e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan  4 21:22:03 UTC 2019
