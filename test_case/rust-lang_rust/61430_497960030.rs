plain
travis_time:end:2d691168:start=1559404621976588641,finish=1559404623755202286,duration=1778613645
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:47:04]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:48:08]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:48:08]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:49:10]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:49:12] error: internal compiler error: src/librustc_mir/transform/generator.rs:532: Broken MIR: generator contains type passes::ExpansionResult in MIR, but typeck only knows about for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6> {std::rc::Rc<rustc::session::Session>, &'r std::rc::Rc<rustc::session::Session>, rustc::session::Session, &'s rustc::session::Session, rustc_metadata::creader::CrateLoader<'t0>, rustc_resolve::ResolverArenas<'t1>, std::result::Result<(syntax::ast::Crate, rustc_resolve::Resolver<'t2>), rustc::util::common::ErrorReported>, rustc::util::common::ErrorReported, fn(std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>) -> rustc_data_structures::box_region::YieldType<std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, for<'t7, 't8> fn((&'t7 mut rustc_resolve::Resolver<'t8>,))> {passes::BoxedResolver::initial_yield}, fn(rustc::util::common::ErrorReported) -> std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported> {std::result::Result::<syntax::ast::Crate, rustc::util::common::ErrorReported>::Err}, std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, rustc_data_structures::box_region::YieldType<std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, for<'t7, 't8> fn((&'t7 mut rustc_resolve::Resolver<'t8>,))>, (), syntax::ast::Crate, rustc_resolve::Resolver<'t3>, fn(syntax::ast::Crate) -> std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported> {std::result::Result::<syntax::ast::Crate, rustc::util::common::ErrorReported>::Ok}, std::thread::LocalKey<std::cell::Cell<rustc_data_structures::box_region::Action>>, &'t4 std::thread::LocalKey<std::cell::Cell<rustc_data_structures::box_region::Action>>, [closure@<::rustc_data_structures::box_region::box_region_allow_access macros>:5:56: 5:74], rustc_data_structures::box_region::Action, rustc_data_structures::box_region::AccessAction, &'t5 mut (dyn for<'t7, 't8> std::ops::FnMut(&'t7 mut rustc_resolve::Resolver<'t8>) + 't6), rustc_data_structures::box_region::Marker<for<'t7, 't8> fn((&'t7 mut rustc_resolve::Resolver<'t8>,))>, fn(rustc_data_structures::box_region::Marker<for<'t7, 't8> fn((&'t7 mut rustc_resolve::Resolver<'t8>,))>) -> rustc_data_structures::box_region::YieldType<std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, for<'t7, 't8> fn((&'t7 mut rustc_resolve::Resolver<'t8>,))> {rustc_data_structures::box_region::YieldType::<std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, for<'t7, 't8> fn((&'t7 mut rustc_resolve::Resolver<'t8>,))>::Accessor}}
[00:49:12]    --> src/librustc_interface/passes.rs:140:49
[00:49:12]     |
[00:49:12] 140 |       let (result, resolver) = BoxedResolver::new(static move || {
[00:49:12]     |  _________________________________________________^
[00:49:12] 141 | |         let sess = &*sess;
[00:49:12] 142 | |         let mut crate_loader = CrateLoader::new(sess, &*cstore, &crate_name);
[00:49:12] 143 | |         let resolver_arenas = Resolver::arenas();
[00:49:12] 164 | |         ExpansionResult::from_owned_resolver(resolver)
[00:49:12] 165 | |     });
[00:49:12]     | |_____^
[00:49:12] 
---
[00:49:12] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:49:12] 
[00:49:12] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:49:12] 
[00:49:12] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:49:12] note: some of the compiler flags provided by cargo are hidden
[00:49:12] 
[00:49:12] error: Could not compile `rustc_interface`.
[00:49:12] 
---
travis_time:end:03fda2e0:start=1559407588230531836,finish=1559407588235205295,duration=4673459
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0be1d98e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0785f9bd
travis_time:start:0785f9bd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:022a6a78
$ dmesg | grep -i kill
