plain
travis_time:end:03bcc2da:start=1553016411285117046,finish=1553016413719795677,duration=2434678631
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:02]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:27]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:29]    Compiling synstructure v0.10.1
[00:06:43]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:08] error[E0277]: `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:07:08]      |
[00:07:08]      |
[00:07:08] 2709 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:07:08]      |                                                ^^^^^^^^ `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:07:08]      |
[00:07:08]      = help: the trait `std::marker::Send` is not implemented for `(dyn std::any::Any + std::marker::Sync + 'static)`
[00:07:08]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:07:08]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:07:08]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:07:08]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:07:08]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:07:08]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:07:08]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:07:08]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:07:08]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:07:08]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:07:08]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2709:57: 2711:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, 'tcx>]`
[00:07:08] 
[00:07:12] error[E0277]: `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:07:12]     --> src/librustc/ty/context.rs:1994:13
[00:07:12]      |
[00:07:12] 1994 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:07:12]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::any::Any + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:07:12]      |
[00:07:12]      = help: the trait `std::marker::Send` is not implemented for `(dyn std::any::Any + std::marker::Sync + 'static)`
[00:07:12]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:07:12]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>`
[00:07:12]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:07:12]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:07:12]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>>`
[00:07:12]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + std::marker::Sync + 'static)>>`
[00:07:12]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:07:12]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:07:12]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:07:12]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:07:12]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:07:13] error: aborting due to 2 previous errors
[00:07:13] 
[00:07:13] For more information about this error, try `rustc --explain E0277`.
[00:07:13] error: Could not compile `rustc`.
---
travis_time:end:03edd70a:start=1553016861141598720,finish=1553016861147320247,duration=5721527
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ac9c5fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0055bd30
travis_time:start:0055bd30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06efd507
$ dmesg | grep -i kill
