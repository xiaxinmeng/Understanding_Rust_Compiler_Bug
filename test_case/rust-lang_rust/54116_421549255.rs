plain
[01:05:50]    Compiling clippy_dev v0.0.1 (file:///checkout/src/tools/clippy/clippy_dev)
[01:05:51]    Compiling syn v0.13.11
[01:06:09]    Compiling derive-new v0.5.4
[01:06:14]    Compiling clippy v0.0.212 (file:///checkout/src/tools/clippy)
[01:06:15] error[E0433]: failed to resolve. Could not find `syntax` in `{{root}}`
[01:06:15]   --> tools/clippy/tests/matches.rs:10:9
[01:06:15] 10 |     use syntax::source_map::DUMMY_SP;
[01:06:15] 10 |     use syntax::source_map::DUMMY_SP;
[01:06:15]    |         ^^^^^^ Could not find `syntax` in `{{root}}`
[01:06:15] 
[01:06:15] error[E0425]: cannot find value `DUMMY_SP` in this scope
[01:06:15]   --> tools/clippy/tests/matches.rs:13:15
[01:06:15] 13 |         span: DUMMY_SP,
[01:06:15]    |               ^^^^^^^^ not found in this scope
[01:06:15] 
[01:06:15] warning: unused import: `syntax::source_map::DUMMY_SP`
---
[01:17:19]    Compiling racer v2.1.5
[01:18:46] error[E0432]: unresolved import `rand`
[01:18:46]   --> tools/rls/src/actions/format.rs:21:5
[01:18:46]    |
[01:18:46] 21 | use rand::{distributions, thread_rng, Rng};
[01:18:46]    |     ^^^^ Could not find `rand` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc` in `{{root}}`
[01:18:46]    |
[01:18:46]    |
[01:18:46] 14 | use rustc::session::config::{self, ErrorOutputType, Input};
[01:18:46]    |     ^^^^^ Could not find `rustc` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc` in `{{root}}`
[01:18:46]    |
[01:18:46] 15 | use rustc::session::Session;
[01:18:46] 15 | use rustc::session::Session;
[01:18:46]    |     ^^^^^ Could not find `rustc` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc_codegen_utils` in `{{root}}`
[01:18:46]    |
[01:18:46] 16 | use rustc_codegen_utils::codegen_backend::CodegenBackend;
[01:18:46] 16 | use rustc_codegen_utils::codegen_backend::CodegenBackend;
[01:18:46]    |     ^^^^^^^^^^^^^^^^^^^ Could not find `rustc_codegen_utils` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc_driver` in `{{root}}`
[01:18:46]    |
[01:18:46] 17 | use rustc_driver::driver::CompileController;
[01:18:46] 17 | use rustc_driver::driver::CompileController;
[01:18:46]    |     ^^^^^^^^^^^^ Could not find `rustc_driver` in `{{root}}`
[01:18:46] error[E0432]: unresolved import `getopts`
[01:18:46]   --> tools/rls/src/build/rustc.rs:10:5
[01:18:46]    |
[01:18:46] 10 | use getopts;
[01:18:46] 10 | use getopts;
[01:18:46]    |     ^^^^^^^ no `getopts` external crate
[01:18:46] error[E0432]: unresolved import `rustc_driver`
[01:18:46]   --> tools/rls/src/build/rustc.rs:18:5
[01:18:46]    |
[01:18:46]    |
[01:18:46] 18 | use rustc_driver::{run, run_compiler, Compilation, CompilerCalls, RustcDefaultCalls};
[01:18:46]    |     ^^^^^^^^^^^^ Could not find `rustc_driver` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc_metadata` in `{{root}}`
[01:18:46]    |
[01:18:46] 20 | use rustc_metadata::cstore::CStore;
[01:18:46] 20 | use rustc_metadata::cstore::CStore;
[01:18:46]    |     ^^^^^^^^^^^^^^ Could not find `rustc_metadata` in `{{root}}`
[01:18:46] error[E0432]: unresolved import `rustc_errors`
[01:18:46]   --> tools/rls/src/build/rustc.rs:19:5
[01:18:46]    |
[01:18:46] 19 | use rustc_errors;
[01:18:46] 19 | use rustc_errors;
[01:18:46]    |     ^^^^^^^^^^^^ no `rustc_errors` external crate
[01:18:46] error[E0432]: unresolved import `rustc_resolve`
[01:18:46]   --> tools/rls/src/build/rustc.rs:21:5
[01:18:46]    |
[01:18:46] 21 | use rustc_resolve;
[01:18:46] 21 | use rustc_resolve;
[01:18:46]    |     ^^^^^^^^^^^^^ no `rustc_resolve` external crate
[01:18:46] error[E0432]: unresolved import `rustc_save_analysis`
[01:18:46]   --> tools/rls/src/build/rustc.rs:22:5
[01:18:46]    |
[01:18:46] 22 | use rustc_save_analysis as save;
[01:18:46] 22 | use rustc_save_analysis as save;
[01:18:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `rustc_save_analysis` external crate
[01:18:46] error[E0432]: unresolved import `rustc_save_analysis`
[01:18:46]   --> tools/rls/src/build/rustc.rs:23:5
[01:18:46]    |
[01:18:46]    |
[01:18:46] 23 | use rustc_save_analysis::CallbackHandler;
[01:18:46]    |     ^^^^^^^^^^^^^^^^^^^ Could not find `rustc_save_analysis` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `syntax` in `{{root}}`
[01:18:46]    |
[01:18:46] 25 | use syntax::source_map::{FileLoader, RealFileLoader};
[01:18:46] 25 | use syntax::source_map::{FileLoader, RealFileLoader};
[01:18:46]    |     ^^^^^^ Could not find `syntax` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc_plugin` in `{{root}}`
[01:18:46]     |
[01:18:46] 150 |     use rustc_plugin::registry::Registry;
[01:18:46] 150 |     use rustc_plugin::registry::Registry;
[01:18:46]     |         ^^^^^^^^^^^^ Could not find `rustc_plugin` in `{{root}}`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Could not find `rustc_driver` in `{{root}}`
[01:18:46]     |
[01:18:46]     |
[01:18:46] 149 | fn clippy_after_parse_callback(state: &mut ::rustc_driver::driver::CompileState<'_, '_>) {
[01:18:46]     |                                              ^^^^^^^^^^^^ Could not find `rustc_driver` in `{{root}}`
[01:18:46] error[E0433]: failed to resolve. Use of undeclared type or module `Registry`
[01:18:46]    --> tools/rls/src/build/rustc.rs:152:24
[01:18:46]     |
[01:18:46] 152 |     let mut registry = Registry::new(
[01:18:46] 152 |     let mut registry = Registry::new(
[01:18:46]     |                        ^^^^^^^^ Use of undeclared type or module `Registry`
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Use of undeclared type or module `config`
[01:18:46]    --> tools/rls/src/build/rustc.rs:196:17
[01:18:46]     |
[01:18:46] 196 |         sopts: &config::Options,
[01:18:46] 
[01:18:46] error[E0433]: failed to resolve. Use of undeclared type or module `config`
[01:18:46]    --> tools/rls/src/build/rustc.rs:208:17
[01:18:46]     |
[01:18:46]     |
[01:18:46] 208 |         sopts: &config::Options,
[01:18:46] 
[01:18:46] error[E0422]: cannot find struct, variant or union type `Registry` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:167:9
[01:18:46]     |
[01:18:46]     |
[01:18:46] 167 |     let Registry {
[01:18:46]     |         ^^^^^^^^ not found in this scope
[01:18:46] 
[01:18:46] error[E0412]: cannot find type `ErrorOutputType` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:199:17
[01:18:46]     |
[01:18:46] 199 |         output: ErrorOutputType,
[01:18:46] 
[01:18:46] error[E0412]: cannot find type `Input` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:213:18
[01:18:46]     |
[01:18:46]     |
[01:18:46] 213 |     ) -> Option<(Input, Option<PathBuf>)> {
[01:18:46] help: possible candidates are found in other modules, you can import them into scope
[01:18:46]     |
[01:18:46] 10  | use regex::internal::Input;
[01:18:46]     |
---
[01:18:46] help: possible candidates are found in other modules, you can import them into scope
[01:18:46]     |
[01:18:46] 10  | use racer::Session;
[01:18:46]     |
[01:18:46] 10  | use rustfmt_nightly::Session;
[01:18:46] 
[01:18:46] error[E0412]: cannot find type `CStore` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:223:18
[01:18:46]     |
[01:18:46]     |
[01:18:46] 223 |         cstore: &CStore,
[01:18:46] 
[01:18:46] error[E0412]: cannot find type `Input` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:224:17
[01:18:46]     |
---
[01:18:46] help: possible candidates are found in other modules, you can import them into scope
[01:18:46]     |
[01:18:46] 10  | use racer::Session;
[01:18:46]     |
[01:18:46] 10  | use rustfmt_nightly::Session;
[01:18:46] 
[01:18:46] error[E0412]: cannot find type `CompileController` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:237:10
[01:18:46]     |
---
[01:18:46] 
[01:18:46] error[E0405]: cannot find trait `FileLoader` in this scope
[01:18:46]    --> tools/rls/src/build/rustc.rs:303:6
[01:18:46]     |
[01:18:46] 303 | impl FileLoader for ReplacedFileLoader {
[01:18:46] help: possible candidate is found in another module, you can import it into scope
[01:18:46]     |
[01:18:46] 10  | use racer::FileLoader;
[01:18:46]     |
[01:18:46]     |
[01:18:46] 
[01:18:46] warning: unused import: `Rng`
[01:18:46]   --> tools/rls/src/actions/format.rs:21:39
[01:18:46]    |
[01:18:46] 21 | use rand::{distributions, thread_rng, Rng};
[01:18:46]    |
[01:18:46]    = note: #[warn(unused_imports)] on by default
[01:18:46] 
[01:18:46] 
[01:18:46] warning: unused imports: `ErrorOutputType`, `Input`, `self`
[01:18:46]    |
[01:18:46]    |
[01:18:46] 14 | use rustc::session::config::{self, ErrorOutputType, Input};
[01:18:46] 
[01:18:46] warning: unused import: `rustc::session::Session`
[01:18:46]   --> tools/rls/src/build/rustc.rs:15:5
[01:18:46]    |
---
[01:18:46]    |
[01:18:46] 20 | use rustc_metadata::cstore::CStore;
[01:18:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:18:46] 
[01:18:46] warning: unused imports: `FileLoader`, `RealFileLoader`
[01:18:46]    |
[01:18:46] 25 | use syntax::source_map::{FileLoader, RealFileLoader};
[01:18:46]    |                          ^^^^^^^^^^  ^^^^^^^^^^^^^^
[01:18:46] 
---
[01:24:05] Verifying status of rustfmt...
[01:24:05] Verifying status of clippy-driver...
[01:24:05] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:24:05] 
[01:24:05] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:24:05] 
[01:24:05] If you do intend to update 'clippy-driver', please check the error messages above and
[01:24:05] commit another update.
[01:24:05] 
[01:24:05] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:24:05] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:24:05] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:07bd0dc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0590c1a8:start=1537005986694360247,finish=1537005986700194062,duration=5833815
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14d982f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c20e2f9
travis_time:start:1c20e2f9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e41de74
$ dmesg | grep -i kill
