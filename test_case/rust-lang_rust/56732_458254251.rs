plain
travis_time:end:11488ed8:start=1548699787547714919,finish=1548699790022165881,duration=2474450962
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:47]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:25:34]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:25:34]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:25:34]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:25:34] error[E0599]: no method named `resume` found for type `std::boxed::Box<(dyn std::ops::Generator<Return=passes::ExpansionResult, Yield=rustc_data_structures::box_region::YieldType<std::result::Result<syntax::ast::Crate, rustc::util::common::ErrorReported>, for<'r, 's> fn((&'r mut rustc_resolve::Resolver<'s>,))>> + 'static)>` in the current scope
[00:25:34]    --> <::rustc_data_structures::box_region::box_region_type_new macros>:4:29
[00:25:34]     |
[00:25:34] 1   | / ( $ name : path , $ code : block ) => {
[00:25:34] 2   | | {
[00:25:34] 3   | | let mut result = $ name ( Box :: new ( static move || $ code ) ) ; let init =
[00:25:34] 4   | | match unsafe { result . 0 . resume (  ) } {
[00:25:34]     | |                             ^^^^^^
[00:25:34] 5   | | :: std :: ops :: GeneratorState :: Yielded (
[00:25:34] 6   | | $ crate :: box_region :: YieldType :: Initial ( y ) ) => y , _ => panic ! (  )
[00:25:34] 7   | | } ; ( init , result ) } }
[00:25:34]     | |_________________________- in this expansion of `box_region_type_new!`
[00:25:34]    ::: src/librustc_interface/passes.rs:133:30
[00:25:35]     |
[00:25:35]     |
[00:25:35] 133 |       let (result, resolver) = box_region_type_new!(BoxedResolver, {
[00:25:35]     |  ______________________________-
[00:25:35] 134 | |         let sess = &*sess;
[00:25:35] 135 | |         let mut crate_loader = CrateLoader::new(sess, &*cstore, &crate_name);
[00:25:35] 136 | |         let resolver_arenas = Resolver::arenas();
[00:25:35] 156 | |         ExpansionResult::from_owned_resolver(resolver)
[00:25:35] 157 | |     });
[00:25:35]     | |______- in this macro invocation
[00:25:35] 
[00:25:35] 
[00:25:35] error[E0599]: no method named `resume` found for type `std::boxed::Box<dyn std::ops::Generator<Return=(), Yield=rustc_data_structures::box_region::YieldType<(), for<'r, 's> fn((&'r rustc::hir::map::Map<'s>,))>>>` in the current scope
[00:25:35]    --> <::rustc_data_structures::box_region::box_region_type_new macros>:4:29
[00:25:35]     |
[00:25:35] 1   | / ( $ name : path , $ code : block ) => {
[00:25:35] 2   | | {
[00:25:35] 3   | | let mut result = $ name ( Box :: new ( static move || $ code ) ) ; let init =
[00:25:35] 4   | | match unsafe { result . 0 . resume (  ) } {
[00:25:35]     | |                             ^^^^^^
[00:25:35] 5   | | :: std :: ops :: GeneratorState :: Yielded (
[00:25:35] 6   | | $ crate :: box_region :: YieldType :: Initial ( y ) ) => y , _ => panic ! (  )
[00:25:35] 7   | | } ; ( init , result ) } }
[00:25:35]     | |_________________________- in this expansion of `box_region_type_new!`
[00:25:35]    ::: src/librustc_interface/passes.rs:791:24
[00:25:35]     |
[00:25:35]     |
[00:25:35] 791 |       let ((), result) = box_region_type_new!(BoxedHirMap, {
[00:25:35] 792 | |         let sess = sess;
[00:25:35] 792 | |         let sess = sess;
[00:25:35] 793 | |         let cstore = cstore;
[00:25:35] 794 | |         let mut hir_forest = hir_forest;
[00:25:35] ...   |
[00:25:35] 802 | |         box_region_allow_access!(for(), (&hir::map::Map<'_>), (&hir_map));
[00:25:35] 803 | |     });
[00:25:35] 
[00:25:35] 
[00:25:35] error[E0599]: no method named `resume` found for type `std::boxed::Box<dyn std::ops::Generator<Return=(), Yield=rustc_data_structures::box_region::YieldType<(), for<'gcx> fn((&'gcx rustc::ty::GlobalCtxt<'gcx>,))>>>` in the current scope
[00:25:35]    --> <::rustc_data_structures::box_region::box_region_type_new macros>:4:29
[00:25:35]     |
[00:25:35] 1   | / ( $ name : path , $ code : block ) => {
[00:25:35] 2   | | {
[00:25:35] 3   | | let mut result = $ name ( Box :: new ( static move || $ code ) ) ; let init =
[00:25:35] 4   | | match unsafe { result . 0 . resume (  ) } {
[00:25:35]     | |                             ^^^^^^
[00:25:35] 5   | | :: std :: ops :: GeneratorState :: Yielded (
[00:25:35] 6   | | $ crate :: box_region :: YieldType :: Initial ( y ) ) => y , _ => panic ! (  )
[00:25:35] 7   | | } ; ( init , result ) } }
[00:25:35]     | |_________________________- in this expansion of `box_region_type_new!`
[00:25:35]    ::: src/librustc_interface/passes.rs:840:24
[00:25:35]     |
[00:25:35]     |
[00:25:35] 840 |       let ((), result) = box_region_type_new!(BoxedGlobalCtxt, {
[00:25:35] 841 | |         let sess = sess;
[00:25:35] 841 | |         let sess = sess;
[00:25:35] 842 | |         let cstore = cstore;
[00:25:35] 843 | |         let mut hir_forest = hir_forest;
[00:25:35] 895 | |         }
[00:25:35] 896 | |     });
[00:25:35]     | |______- in this macro invocation
[00:25:35] 
