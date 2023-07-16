plain
travis_time:end:1b0a1dc2:start=1546564889746272671,finish=1546564890831991361,duration=1085718690
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:23:26] 
[01:23:26] ------------------------------------------
[01:23:26] stderr:
[01:23:26] ------------------------------------------
[01:23:26] {"message":"src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])\n\n"}
[01:23:26] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
[01:23:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:26] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:23:26] note: the compiler unexpectedly panicked. this is a bug.
[01:23:26] 
[01:23:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:23:26] 
---
[01:31:36]     |
[01:31:36] 151 |                 .position(|line| line.trim_right().ends_with("*/"))
[01:31:36]     |                                       ^^^^^^^^^^
[01:31:36] 
[01:33:11] warning[E0597]: `msg.method` does not live long enough
[01:33:11]     |
[01:33:11] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:33:11] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:33:11]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:33:11] 260 |             msg.method;
[01:33:11] 260 |             msg.method;
[01:33:11]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:33:11] 290 |     }
[01:33:11] 290 |     }
[01:33:11]     |     - `msg.method` dropped here while still borrowed
[01:33:11]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:33:11]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:33:11] 
[01:35:24]     Finished release [optimized] target(s) in 11m 57s
[01:35:24]     Finished release [optimized] target(s) in 11m 57s
[01:35:24] travis_fold:end:stage2-rls

[01:35:24] travis_time:end:stage2-rls:start=1546569906365279507,finish=1546570623924231275,duration=717558951768

[01:35:24]    Compiling difference v2.0.0
[01:35:25]    Compiling rls v1.31.6 (/checkout/src/tools/rls)
[01:35:36] warning[E0597]: `msg.method` does not live long enough
[01:35:36]     |
[01:35:36] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:35:36] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:35:36]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:35:36] 260 |             msg.method;
[01:35:36] 260 |             msg.method;
[01:35:36]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:35:36] 290 |     }
[01:35:36] 290 |     }
[01:35:36]     |     - `msg.method` dropped here while still borrowed
[01:35:36]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:35:36]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:35:36] 
[01:37:53]     Finished release [optimized] target(s) in 2m 29s
