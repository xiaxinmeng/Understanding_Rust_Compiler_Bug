plain
travis_time:end:01b24c74:start=1549605404487616446,finish=1549605405581896858,duration=1094280412
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:06]    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
[01:01:06] error[E0433]: failed to resolve: use of undeclared type or module `syntax`
[01:01:06]   --> src/tools/error_index_generator/main.rs:17:5
[01:01:06]    |
[01:01:06] 17 | use syntax::diagnostics::metadata::{get_metadata_dir, ErrorMetadataMap, ErrorMetadata};
[01:01:06] 
[01:01:06] 
[01:01:07] error[E0433]: failed to resolve: use of undeclared type or module `env_logger`
[01:01:07]     |
[01:01:07] 257 |     env_logger::init();
[01:01:07] 257 |     env_logger::init();
[01:01:07]     |     ^^^^^^^^^^ use of undeclared type or module `env_logger`
[01:01:07] error[E0433]: failed to resolve: use of undeclared type or module `syntax`
[01:01:07]    --> src/tools/error_index_generator/main.rs:262:18
[01:01:07]     |
[01:01:07]     |
[01:01:07] 262 |     let result = syntax::with_globals(move || {
[01:01:07] 
[01:01:07] 
[01:01:07] error[E0412]: cannot find type `ErrorMetadata` in this scope
[01:01:07]    |
[01:01:07]    |
[01:01:07] 41 |     fn error_code_block(&self, output: &mut dyn Write, info: &ErrorMetadata,
[01:01:07] 
[01:01:07] 
[01:01:07] error[E0412]: cannot find type `ErrorMetadata` in this scope
[01:01:07]    |
[01:01:07]    |
[01:01:07] 75 |     fn error_code_block(&self, output: &mut dyn Write, info: &ErrorMetadata,
[01:01:07] 
[01:01:07] 
[01:01:07] error[E0412]: cannot find type `ErrorMetadata` in this scope
[01:01:07]     |
[01:01:07]     |
[01:01:07] 182 |     fn error_code_block(&self, output: &mut dyn Write, info: &ErrorMetadata,
[01:01:07] 
[01:01:07] 
[01:01:07] error[E0412]: cannot find type `ErrorMetadataMap` in this scope
[01:01:07]     |
[01:01:07]     |
[01:01:07] 197 | fn load_all_errors(metadata_dir: &Path) -> Result<ErrorMetadataMap, Box<dyn Error>> {
[01:01:07] 
[01:01:07] 
[01:01:07] error[E0412]: cannot find type `ErrorMetadataMap` in this scope
[01:01:07]     |
[01:01:07]     |
[01:01:07] 205 |         let some_errors: ErrorMetadataMap = json::decode(&metadata_str)?;
[01:01:07] 
[01:01:07] 
[01:01:07] error[E0412]: cannot find type `ErrorMetadataMap` in this scope
[01:01:07]     |
[01:01:07]     |
[01:01:07] 216 | fn render_error_page<T: Formatter>(err_map: &ErrorMetadataMap, output_path: &Path,
[01:01:07] 
[01:01:07] error[E0425]: cannot find function `get_metadata_dir` in this scope
[01:01:07]    --> src/tools/error_index_generator/main.rs:232:24
[01:01:07]     |
[01:01:07]     |
[01:01:07] 232 |     let metadata_dir = get_metadata_dir(&build_arch);
[01:01:07] 
[01:01:07] 
[01:01:07] error: unused imports: `ErrorMetadataMap`, `ErrorMetadata`, `get_metadata_dir`
[01:01:07]    |
[01:01:07]    |
[01:01:07] 17 | use syntax::diagnostics::metadata::{get_metadata_dir, ErrorMetadataMap, ErrorMetadata};
[01:01:07]    |
[01:01:07]    = note: `-D unused-imports` implied by `-D warnings`
[01:01:07] 
[01:01:07] error[E0277]: the size for values of type `str` cannot be known at compilation time
[01:01:07] error[E0277]: the size for values of type `str` cannot be known at compilation time
[01:01:07]   --> src/tools/error_index_generator/main.rs:95:13
[01:01:07]    |
[01:01:07] 95 |             Some(ref desc) => {
[01:01:07]    |
[01:01:07]    = help: the trait `std::marker::Sized` is not implemented for `str`
[01:01:07]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:01:07]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:01:07]    = note: required by `std::prelude::v1::Some`
[01:01:07] 
[01:01:07] error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
[01:01:07]     |
[01:01:07]     |
[01:01:07] 232 |     let metadata_dir = get_metadata_dir(&build_arch);
[01:01:07]     |         ^^^^^^^^^^^^ borrow the `Path` instead
[01:01:07]     |
[01:01:07]     = help: within `std::path::Path`, the trait `std::marker::Sized` is not implemented for `[u8]`
[01:01:07]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[01:01:07]     = note: required because it appears within the type `std::path::Path`
[01:01:07]     = help: unsized locals are gated as an unstable feature
[01:01:07] 
[01:01:07] error: aborting due to 13 previous errors
[01:01:07] 
---
[01:01:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/error_index_generator/Cargo.toml" "--message-format" "json"
[01:01:07] expected success, got: exit code: 101
[01:01:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:01:07] Build completed unsuccessfully in 0:07:00
[01:01:07] make: *** [all] Error 1
[01:01:07] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11f7f0ba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 06:58:04 UTC 2019
---
travis_time:end:12ab7070:start=1549609085753630922,finish=1549609085759602286,duration=5971364
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b57d8e6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travi
