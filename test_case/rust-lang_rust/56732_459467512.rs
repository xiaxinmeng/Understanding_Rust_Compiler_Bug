plain
travis_time:end:091a4960:start=1548960181864586945,finish=1548960185598308405,duration=3733721460
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:47]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:24:38]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:24:38]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:24:38]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:24:39] error[E0599]: no method named `resume` found for type `std::pin::Pin<std::boxed::Box<(dyn std::ops::Generator<Return=passes::ExpansionResult, Yield=rustc_data_structures::box_region::YieldType<std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, for<'r, 's> fn((&'r mut rustc_resolve::Resolver<'s>,))>> + 'static)>>` in the current scope
[00:24:39]    --> <::rustc_data_structures::box_region::box_region_type_new macros>:4:20
[00:24:39]     |
[00:24:39] 1   | / ( $ name : path , $ code : block ) => {
[00:24:39] 2   | | {
[00:24:39] 3   | | let mut result = $ name ( Box :: pin ( static move || $ code ) ) ; let init =
[00:24:39] 4   | | match result . 0 . resume (  ) {
[00:24:39]     | |                    ^^^^^^
[00:24:39] 5   | | :: std :: ops :: GeneratorState :: Yielded (
[00:24:39] 6   | | $ crate :: box_region :: YieldType :: Initial ( y ) ) => y , _ => panic ! (  )
[00:24:39] 7   | | } ; ( init , result ) } }
[00:24:39]     | |_________________________- in this expansion of `box_region_type_new!`
[00:24:39]    ::: src/librustc_interface/passes.rs:133:30
[00:24:39]     |
[00:24:39]     |
[00:24:39] 133 |       let (result, resolver) = box_region_type_new!(BoxedResolver, {
[00:24:39]     |  ______________________________-
[00:24:39] 134 | |         let sess = &*sess;
[00:24:39] 135 | |         let mut crate_loader = CrateLoader::new(sess, &*cstore, &crate_name);
[00:24:39] 136 | |         let resolver_arenas = Resolver::arenas();
[00:24:39] 156 | |         ExpansionResult::from_owned_resolver(resolver)
[00:24:39] 157 | |     });
[00:24:39]     | |______- in this macro invocation
[00:24:39] 
[00:24:39] 
[00:24:39] error[E0599]: no method named `resume` found for type `std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Return=(), Yield=rustc_data_structures::box_region::YieldType<(), for<'r, 's> fn((&'r rustc::hir::map::Map<'s>,))>>>>` in the current scope
[00:24:39]    --> <::rustc_data_structures::box_region::box_region_type_new macros>:4:20
[00:24:39]     |
[00:24:39] 1   | / ( $ name : path , $ code : block ) => {
[00:24:39] 2   | | {
[00:24:39] 3   | | let mut result = $ name ( Box :: pin ( static move || $ code ) ) ; let init =
[00:24:39] 4   | | match result . 0 . resume (  ) {
[00:24:39]     | |                    ^^^^^^
[00:24:39] 5   | | :: std :: ops :: GeneratorState :: Yielded (
[00:24:39] 6   | | $ crate :: box_region :: YieldType :: Initial ( y ) ) => y , _ => panic ! (  )
[00:24:39] 7   | | } ; ( init , result ) } }
[00:24:39]     | |_________________________- in this expansion of `box_region_type_new!`
[00:24:39]    ::: src/librustc_interface/passes.rs:791:24
[00:24:39]     |
[00:24:39]     |
[00:24:39] 791 |       let ((), result) = box_region_type_new!(BoxedHirMap, {
[00:24:39] 792 | |         let sess = sess;
[00:24:39] 792 | |         let sess = sess;
[00:24:39] 793 | |         let cstore = cstore;
[00:24:39] 794 | |         let mut hir_forest = hir_forest;
[00:24:39] ...   |
[00:24:39] 802 | |         box_region_allow_access!(for(), (&hir::map::Map<'_>), (&hir_map));
[00:24:39] 803 | |     });
[00:24:39] 
[00:24:39] 
[00:24:39] error[E0599]: no method named `resume` found for type `std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Return=(), Yield=rustc_data_structures::box_region::YieldType<(), for<'gcx> fn((&'gcx rustc::ty::GlobalCtxt<'gcx>,))>>>>` in the current scope
[00:24:39]    --> <::rustc_data_structures::box_region::box_region_type_new macros>:4:20
[00:24:39]     |
[00:24:39] 1   | / ( $ name : path , $ code : block ) => {
[00:24:39] 2   | | {
[00:24:39] 3   | | let mut result = $ name ( Box :: pin ( static move || $ code ) ) ; let init =
[00:24:39] 4   | | match result . 0 . resume (  ) {
[00:24:39]     | |                    ^^^^^^
[00:24:39] 5   | | :: std :: ops :: GeneratorState :: Yielded (
[00:24:39] 6   | | $ crate :: box_region :: YieldType :: Initial ( y ) ) => y , _ => panic ! (  )
[00:24:39] 7   | | } ; ( init , result ) } }
[00:24:39]     | |_________________________- in this expansion of `box_region_type_new!`
[00:24:39]    ::: src/librustc_interface/passes.rs:840:24
[00:24:39]     |
[00:24:39]     |
[00:24:39] 840 |       let ((), result) = box_region_type_new!(BoxedGlobalCtxt, {
[00:24:39] 841 | |         let sess = sess;
[00:24:39] 841 | |         let sess = sess;
[00:24:39] 842 | |         let cstore = cstore;
[00:24:39] 843 | |         let mut hir_forest = hir_forest;
[00:24:39] 895 | |         }
[00:24:39] 896 | |     });
[00:24:39]     | |______- in this macro invocation
[00:24:39] 
---
[00:25:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:25:59] expected success, got: exit code: 101
[00:25:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:25:59] Build completed unsuccessfully in 0:21:47
[00:25:59] make: *** [all] Error 1
[00:25:59] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13cf85ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 19:09:15 UTC 2019
---
travis_time:end:04ae00b4:start=1548961756328621193,finish=1548961756335434059,duration=6812866
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15587798
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a95d5d0
travis_time:start:0a95d5d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:088fccdc
$ dmesg | grep -i kill
