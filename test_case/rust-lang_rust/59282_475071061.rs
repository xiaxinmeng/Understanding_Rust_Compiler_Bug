plain
travis_time:end:008daebe:start=1553117927151946210,finish=1553118000995970362,duration=73844024152
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:05:32]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:57]    Compiling synstructure v0.10.1
[00:06:03]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:12]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:38] error[E0277]: `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:06:38]      |
[00:06:38]      |
[00:06:38] 2725 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:06:38]      |                                                ^^^^^^^^ `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:06:38]      |
[00:06:38]      = help: the trait `std::marker::Send` is not implemented for `(dyn std::any::Any + std::marker::Sync + 'static)`
[00:06:38]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:06:38]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:06:38]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:06:38]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:06:38]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:06:38]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:06:38]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:06:38]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:06:38]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:06:38]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:06:38]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2725:57: 2727:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, 'tcx>]`
[00:06:38] 
[00:06:42] error[E0277]: `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:06:42]     --> src/librustc/ty/context.rs:2013:13
[00:06:42]      |
[00:06:42] 2013 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:06:42]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:06:42]      |
[00:06:42]      = help: the trait `std::marker::Send` is not implemented for `(dyn std::any::Any + std::marker::Sync + 'static)`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:06:42]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:06:42]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:06:42]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:06:42]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:06:42]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:06:42]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:06:42]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:06:42]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:06:42]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:06:43] error: aborting due to 2 previous errors
[00:06:43] 
[00:06:43] For more information about this error, try `rustc --explain E0277`.
[00:06:44] error: Could not compile `rustc`.
---
travis_time:end:051844c4:start=1553118416024686537,finish=1553118416031127365,duration=6440828
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f82baf8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29e5c454
travis_time:start:29e5c454
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f93712
$ dmesg | grep -i kill
