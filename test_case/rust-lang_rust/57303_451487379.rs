plain
travis_time:end:1f2dece7:start=1546611275455427092,finish=1546611276348192484,duration=892765392
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[00:01:20] Successfully built 436eb7c5c4d9
[00:01:20] Successfully tagged rust-ci:latest
[00:01:21] Built container sha256:436eb7c5c4d95fed80e9a7e49f602e11759293d05a7f8c60eb2ef275846504db
[00:01:21] Uploading finished image to s3://rust-lang-ci-sccache2/docker/48065f37e2dcfe7aafa9eba84685b6850bc3ca38c11c5051723c57f394f93f551c801d606e5e0deafa48821df08a27ae54e28a1d481dde3d26967b0e12e72c6c
[00:01:52] upload failed: - to s3://rust-lang-ci-sccache2/docker/48065f37e2dcfe7aafa9eba84685b6850bc3ca38c11c5051723c57f394f93f551c801d606e5e0deafa48821df08a27ae54e28a1d481dde3d26967b0e12e72c6c Unable to locate credentials

[00:01:52] travis_time:end:2a4eda6a:start=1546611300551521661,finish=1546611400932691052,duration=100381169391
[CI_JOB_NAME=x86_64-gnu-tools]
[00:01:52] [CI_JOB_NAME=x86_64-gnu-tools]
---
[01:14:17] failures:
[01:14:17] 
[01:14:17] ---- /checkout/src/doc/rust-by-example/src/attribute/crate.md - Crates (line 12) stdout ----
[01:14:17] thread '/checkout/src/doc/rust-by-example/src/attribute/crate.md - Crates (line 12)' panicked at 'couldn't run the test: No such file or directory (os error 2)', src/librustdoc/test.rs:343:19
[01:14:17] stack backtrace:
[01:14:17]    0:     0x7feab9d05d93 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hd98426cd3492fad9
[01:14:17]    1:     0x7feab9cfdd28 - std::sys_common::backtrace::_print::h33dc68ee3471107b
[01:14:17]    2:     0x7feab9d01b91 - std::panicking::default_hook::{{closure}}::h6e8ca1fbae609b31
[01:14:17]    3:     0x7feab9d01804 - std::panicking::default_hook::h12d08263585ea668
[01:14:17]    4:     0x7feabc0958ef - rustc::util::common::panic_hook::h8664254e5d78c027
[01:14:17]    5:     0x7feab9d024a4 - std::panicking::rust_panic_with_hook::h80ecd07d9fdfd689
[01:14:17]    6:     0x7feab9d01f41 - std::panicking::continue_panic_fmt::h57621e53f49b8d64
[01:14:17]    7:     0x7feab9d01e8e - std::panicking::begin_panic_fmt::h93a0a0ce41036878
[01:14:17]    8:     0x55de5585ac6f - rustdoc::test::run_test::h3f9d6d70740c40c7
[01:14:17]    9:     0x55de555b27c0 - <scoped_tls::ScopedKey<T>>::set::h12fd0d32c5ba3f2d
[01:14:17]   10:     0x55de55723cff - syntax::with_globals::h8b1226f6312a8d21
[01:14:17]   11:     0x55de55470ba4 - std::sys_common::backtrace::__rust_begin_short_backtrace::h6788ddba6b9cfc75
[01:14:17]   12:     0x55de55503355 - std::panicking::try::do_call::h5614d150e98901fd
[01:14:17]   13:     0x7feab9d155f9 - __rust_maybe_catch_panic
[01:14:17]   14:     0x55de55518227 - <F as alloc::boxed::FnBox<A>>::call_box::h00ff5dd798463fcb
[01:14:17]   15:     0x7feab9d1423d - std::sys::unix::thread::Thread::new::thread_start::h013506ce4de301ef
[01:14:17]   16:     0x7feab98726b9 - start_thread
[01:14:17]   17:     0x7feab939241c - clone
[01:14:17]   18:                0x0 - <unknown>
[01:14:17] query stack during panic:
[01:14:17] end of query stack
[01:14:17] 
[01:14:17] failures:
[01:14:17]     /checkout/src/doc/rust-by-example/src/attribute/crate.md - Crates (line 12)
[01:14:17] 
---
[01:28:27] 
[01:28:27] ------------------------------------------
[01:28:27] stderr:
[01:28:27] ------------------------------------------
[01:28:27] {"message":"src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])\n\n"}
[01:28:27] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
[01:28:27] stack backtrace:
[01:28:27]    0:     0x7fe8334a8d93 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hd98426cd3492fad9
[01:28:27]    1:     0x7fe8334a0d28 - std::sys_common::backtrace::_print::h33dc68ee3471107b
[01:28:27]    2:     0x7fe8334a4b91 - std::panicking::default_hook::{{closure}}::h6e8ca1fbae609b31
[01:28:27]    3:     0x7fe8334a4868 - std::panicking::default_hook::h12d08263585ea668
[01:28:27]    4:     0x7fe8355ba8ef - rustc::util::common::panic_hook::h8664254e5d78c027
[01:28:27]    5:     0x7fe8334a54a4 - std::panicking::rust_panic_with_hook::h80ecd07d9fdfd689
[01:28:27]    6:     0x7fe834319e4c - std::panicking::begin_panic::h606da2437d0c9a78
[01:28:27]    7:     0x7fe83434220e - rustc_errors::Handler::bug::h5f18c9aa3002a3e1
[01:28:27]    8:     0x7fe8350964a3 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::ha70b54c46e545966
[01:28:27]    9:     0x7fe835091a59 - rustc::ty::context::tls::with_opt::{{closure}}::h06335bf584d257c5
[01:28:27]   10:     0x7fe835091974 - rustc::ty::context::tls::with_context_opt::hb4bad76e7a1bb6a7
[01:28:27]   11:     0x7fe835091a06 - rustc::ty::context::tls::with_opt::hd45864d4b45a4b65
[01:28:27]   12:     0x7fe835096384 - rustc::util::bug::opt_span_bug_fmt::h1c27e2cc1d1149c3
[01:28:27]   13:     0x7fe8350962f6 - rustc::util::bug::bug_fmt::h8b515233d6f03333
[01:28:27]   14:     0x7fe834e79b45 - rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}::h686eb8a406d530f8
[01:28:27]   15:     0x7fe834e8f648 - rustc::ty::context::tls::with::{{closure}}::h6680e41cb9e76345
[01:28:27]   16:     0x7fe834e8ddff - rustc::ty::context::tls::with_context::{{closure}}::h219fd61af3d2405c
[01:28:27]   17:     0x7fe834e8dda8 - rustc::ty::context::tls::with_context_opt::h390957d2f03df239
[01:28:27]   18:     0x7fe834e8ddd5 - rustc::ty::context::tls::with_context::h180ddfa25e2e2cf2
[01:28:27]   19:     0x7fe834e8f635 - rustc::ty::context::tls::with::hf144144bba27d185
[01:28:27]   20:     0x7fe834e7ab8d - rustc::ty::context::TypeckTables::node_id_to_type_opt::h22a50a087c54f413
[01:28:27]   21:     0x55d06adadaa0 - <clippy_lints::random_state::Pass as rustc::lint::LateLintPass<'a, 'tcx>>::check_ty::h0e791f3e8996c729
[01:28:27]   22:     0x7fe83557bcbf - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_ty::h72ebdaa9528dee36
[01:28:27]   23:     0x7fe835501e2f - rustc::hir::intravisit::walk_item::he489cc90a12bea7b
[01:28:27]   24:     0x7fe83557a108 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h5728335ea922cc15
[01:28:27]   25:     0x7fe83557c64c - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_block::h72db13ed2a88cb28
[01:28:27]   26:     0x7fe83557aa79 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr::hcf6ab50f0c847e80
[01:28:27]   27:     0x7fe835579cdf - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body::h7e2b7e472ece7e94
[01:28:27]   28:     0x7fe835501c2d - rustc::hir::intravisit::walk_item::he489cc90a12bea7b
[01:28:27]   29:     0x7fe83557a108 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h5728335ea922cc15
[01:28:27]   30:     0x7fe83557c0c0 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod::hb9a3c90f0f60db70
[01:28:27]   31:     0x7fe8354f08ae - rustc::hir::intravisit::walk_crate::hde124e87f25baddc
[01:28:27]   32:     0x7fe835582755 - rustc::lint::context::check_crate::h6589d55ae41b1c25
[01:28:27]   33:     0x7fe8365d9d9b - rustc::util::common::time::h7e4a33c762ad15e7
[01:28:27]   34:     0x7fe83667b97a - <std::thread::local::LocalKey<T>>::with::h952b72befc3ce74e
[01:28:27]   35:     0x7fe8365f3a66 - rustc::ty::context::TyCtxt::create_and_enter::h5cc63bc0874b4162
[01:28:27]   36:     0x7fe83665f64e - rustc_driver::driver::compile_input::h03868f75b7316f01
[01:28:27]   37:     0x7fe8365c262a - rustc_driver::run_compiler_with_pool::h741dc60e958f63f8
[01:28:27]   38:     0x7fe8365de9b5 - <scoped_tls::ScopedKey<T>>::set::hc89acbedf3eb8704
[01:28:27]   39:     0x7fe8365c158a - rustc_driver::run_compiler::hce605fc8eee3e069
[01:28:27]   40:     0x55d06acd403c - <scoped_tls::ScopedKey<T>>::set::h9bd1cbe659af503f
[01:28:27]   41:     0x55d06acbdaa2 - std::sys_common::backtrace::__rust_begin_short_backtrace::h79264eee389336bd
[01:28:27]   42:     0x7fe8334b85f9 - __rust_maybe_catch_panic
[01:28:27]   43:     0x55d06acb5792 - std::panicking::try::h5193e9903c6aa0c4
[01:28:27]   44:     0x55d06acbeed5 - <F as alloc::boxed::FnBox<A>>::call_box::hf882d8e28a85db34
[01:28:27]   45:     0x7fe8334b723d - std::sys::unix::thread::Thread::new::thread_start::h013506ce4de301ef
[01:28:27]   46:     0x7fe83321d6b9 - start_thread
[01:28:27]   47:     0x7fe832d3d41c - clone
[01:28:27]   48:                0x0 - <unknown>
[01:28:27] query stack during panic:
[01:28:27] end of query stack
[01:28:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:28:27] note: the compiler unexpectedly panicked. this is a bug.
[01:28:27] 
[01:28:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:28:27] 
---
[01:28:27] 
[01:28:27] ------------------------------------------
[01:28:27] 
[01:28:27] thread '[run-pass] run-pass/used_underscore_binding_macro.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.18/src/runtest.rs:2632:9
[01:28:27] stack backtrace:
[01:28:27]    0:     0x7f961828ed93 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hd98426cd3492fad9
[01:28:27]    1:     0x7f9618286d28 - std::sys_common::backtrace::_print::h33dc68ee3471107b
[01:28:27]    2:     0x7f961828ab91 - std::panicking::default_hook::{{closure}}::h6e8ca1fbae609b31
[01:28:27]    3:     0x7f961828a804 - std::panicking::default_hook::h12d08263585ea668
[01:28:27]    4:     0x7f961828b391 - std::panicking::rust_panic_with_hook::h80ecd07d9fdfd689
[01:28:27]    5:     0x5646e2a28914 - std::panicking::begin_panic::hadf60e87c9929027
[01:28:27]    6:     0x5646e2a5f2d3 - compiletest_rs::runtest::ProcRes::fatal::hfa2c6389155fd953
[01:28:27]    7:     0x5646e2a5c3c3 - compiletest_rs::runtest::TestCx::fatal_proc_rec::hdd310e827939ab5e
[01:28:27]    8:     0x5646e2a4fd50 - compiletest_rs::runtest::TestCx::run_rpass_test::h19370d3a9fbf18e4
[01:28:27]    9:     0x5646e2a411c1 - compiletest_rs::runtest::run::h998e69dd7ca49ec9
[01:28:27]   10:     0x5646e2a1691f - <F as alloc::boxed::FnBox<A>>::call_box::h1a16d2f0372957d2
[01:28:27]   11:     0x7f961852bcc2 - <F as alloc::boxed::FnBox<A>>::call_box::hebbf092c2f8269e4
[01:28:27]   12:     0x7f961829e5f9 - __rust_maybe_catch_panic
[01:28:27]   13:     0x7f961854a0bc - test::run_test::run_test_inner::{{closure}}::h65af44dd80c046ef
[01:28:27]   14:     0x7f9618521d04 - std::sys_common::backtrace::__rust_begin_short_backtrace::h3b93232a43700f8d
[01:28:27]   15:     0x7f9618522334 - std::panicking::try::do_call::h0a3810c7bb8e06bb
[01:28:27]   16:     0x7f961829e5f9 - __rust_maybe_catch_panic
[01:28:27]   17:     0x7f961852b9ec - <F as alloc::boxed::FnBox<A>>::call_box::h1c57c7ed07371bcf
[01:28:27]   18:     0x7f961829d23d - std::sys::unix::thread::Thread::new::thread_start::h013506ce4de301ef
[01:28:27]   19:     0x7f96180036b9 - start_thread
[01:28:27]   20:     0x7f9617b2341c - clone
[01:28:27]   21:                0x0 - <unknown>
[01:28:27] 
[01:28:27] failures:
[01:28:27]     [run-pass] run-pass/used_underscore_binding_macro.rs
[01:28:27] 
---
[01:28:27] failures:
[01:28:27] 
[01:28:27] ---- compile_test stdout ----
[01:28:27] thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.18/src/lib.rs:90:22
[01:28:27] stack backtrace:
[01:28:27]    0:     0x7f961828ed93 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hd98426cd3492fad9
[01:28:27]    1:     0x7f9618286d28 - std::sys_common::backtrace::_print::h33dc68ee3471107b
[01:28:27]    2:     0x7f961828ab91 - std::panicking::default_hook::{{closure}}::h6e8ca1fbae609b31
[01:28:27]    3:     0x7f961828a804 - std::panicking::default_hook::h12d08263585ea668
[01:28:27]    4:     0x7f961828b391 - std::panicking::rust_panic_with_hook::h80ecd07d9fdfd689
[01:28:27]    5:     0x5646e2a28914 - std::panicking::begin_panic::hadf60e87c9929027
[01:28:27]    6:     0x5646e2a17829 - compiletest_rs::run_tests::hf00c144629d2c324
[01:28:27]    7:     0x5646e2a0f492 - core::ops::function::FnOnce::call_once::h37659e959656fa8d
[01:28:27]    8:     0x7f961852b8be - <F as alloc::boxed::FnBox<A>>::call_box::h0134f4ddfc3abdce
[01:28:27]    9:     0x7f961829e5f9 - __rust_maybe_catch_panic
[01:28:27]   10:     0x7f961854a0bc - test::run_test::run_test_inner::{{closure}}::h65af44dd80c046ef
[01:28:27]   11:     0x7f9618521d04 - std::sys_common::backtrace::__rust_begin_short_backtrace::h3b93232a43700f8d
[01:28:27]   12:     0x7f9618522334 - std::panicking::try::do_call::h0a3810c7bb8e06bb
[01:28:27]   13:     0x7f961829e5f9 - __rust_maybe_catch_panic
[01:28:27]   14:     0x7f961852b9ec - <F as alloc::boxed::FnBox<A>>::call_box::h1c57c7ed07371bcf
[01:28:27]   15:     0x7f961829d23d - std::sys::unix::thread::Thread::new::thread_start::h013506ce4de301ef
[01:28:27]   16:     0x7f96180036b9 - start_thread
[01:28:27]   17:     0x7f9617b2341c - clone
[01:28:27]   18:                0x0 - <unknown>
[01:28:27] 
[01:28:27] failures:
[01:28:27]     compile_test
[01:28:27] 
---
[01:37:15]     |
[01:37:15] 151 |                 .position(|line| line.trim_right().ends_with("*/"))
[01:37:15]     |                                       ^^^^^^^^^^
[01:37:15] 
[01:38:55] warning[E0597]: `msg.method` does not live long enough
[01:38:55]     |
[01:38:55] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:38:55] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:38:55]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:38:55] 260 |             msg.method;
[01:38:55] 260 |             msg.method;
[01:38:55]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:38:55] 290 |     }
[01:38:55] 290 |     }
[01:38:55]     |     - `msg.method` dropped here while still borrowed
[01:38:55]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:38:55]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:38:55] 
[01:41:18]     Finished release [optimized] target(s) in 12m 51s
[01:41:18]     Finished release [optimized] target(s) in 12m 51s
[01:41:18] travis_fold:end:stage2-rls

[01:41:18] travis_time:end:stage2-rls:start=1546616595730297973,finish=1546617366929285820,duration=771198987847

[01:41:18]    Compiling difference v2.0.0
[01:41:20]    Compiling rls v1.31.6 (/checkout/src/tools/rls)
[01:41:31] warning[E0597]: `msg.method` does not live long enough
[01:41:31]     |
[01:41:31] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:41:31] 185 |                     <$n_action as LSPNotification>::METHOD => {
[01:41:31]     |                     -------------------------------------- type annotation requires that `msg.method` is borrowed for `'static`
[01:41:31] 260 |             msg.method;
[01:41:31] 260 |             msg.method;
[01:41:31]     |             ^^^^^^^^^^ borrowed value does not live long enough
[01:41:31] 290 |     }
[01:41:31] 290 |     }
[01:41:31]     |     - `msg.method` dropped here while still borrowed
[01:41:31]     = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:41:31]     = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:41:31] 
[01:44:02]     Finished release [optimized] target(s) in 2m 43s
---
[01:53:18] Verifying status of rustfmt...
[01:53:18] Verifying status of clippy-driver...
[01:53:18] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:53:18] 
[01:53:18] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:53:18] 
[01:53:18] If you do intend to update 'clippy-driver', please check the error messages above and
[01:53:18] commit another update.
[01:53:18] 
[01:53:18] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:53:18] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:53:18] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:00df6287
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan  4 16:08:07 UTC 2019
---
travis_time:end:15145d97:start=1546618088815997086,finish=1546618088824097279,duration=8100193
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02a70cde
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01bf8102
$ dmesg | grep -i kill
