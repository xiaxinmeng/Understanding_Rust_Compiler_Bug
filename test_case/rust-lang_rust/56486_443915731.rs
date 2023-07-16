plain
travis_time:end:0932e2f0:start=1543879274842536031,finish=1543879276088840360,duration=1246304329
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:29:15] error: unsatisfied lifetime constraints
[00:29:15]    --> src/librustc_typeck/check/wfcheck.rs:358:5
[00:29:15]     |
[00:29:15] 350 | fn check_item_type<'a, 'tcx>(
[00:29:15]     |                    --  ---- lifetime `'tcx` defined here
[00:29:15]     |                    lifetime `'a` defined here
[00:29:15] ...
[00:29:15] ...
[00:29:15] 358 |     for_id(tcx, item_id, ty_span).with_fcx(|fcx, _this| {
[00:29:15]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
[00:29:15] 
[00:29:15] error[E0716]: temporary value dropped while borrowed
[00:29:15]    --> src/librustc_typeck/check/wfcheck.rs:358:5
[00:29:15] 350 |   fn check_item_type<'a, 'tcx>(
[00:29:15] 350 |   fn check_item_type<'a, 'tcx>(
[00:29:15]     |                          ---- lifetime `'tcx` defined here
[00:29:15] ...
[00:29:15] 358 |       for_id(tcx, item_id, ty_span).with_fcx(|fcx, _this| {
[00:29:15]     |       -^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:29:15]     |       |
[00:29:15]     |  _____creates a temporary which is freed while still in use
[00:29:15]     | |
[00:29:15] 359 | |         let ty = fcx.tcx.type_of(fcx.tcx.hir.local_def_id(item_id));
[00:29:15] 360 | |         let item_ty = fcx.normalize_associated_types_in(ty_span, &ty);
[00:29:15] ...   |
[00:29:15] ...   |
[00:29:15] 378 | |         vec![] // no implied bounds in a const etc
[00:29:15] 379 | |     });
[00:29:15]     | |      -- temporary value is freed at the end of this statement
[00:29:15]     | |______|
[00:29:15]     |        argument requires that borrow lasts for `'tcx`
[00:29:17] error: aborting due to 2 previous errors
[00:29:17] 
[00:29:17] For more information about this error, try `rustc --explain E0716`.
[00:29:17] error: Could not compile `rustc_typeck`.
