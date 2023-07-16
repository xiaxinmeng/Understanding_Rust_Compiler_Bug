plain
travis_time:end:122eb600:start=1549517015290855336,finish=1549517016168785320,duration=877929984
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:21:50]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:22:55] error[E0432]: unresolved import `getopts`
[00:22:55]   --> src/librustc_driver/lib.rs:26:9
[00:22:55]    |
[00:22:55] 26 | pub use getopts;
[00:22:55]    |         ^^^^^^^ no `getopts` external crate
[00:22:55] 
[00:22:55] error: cannot find macro `log_enabled!` in this scope
[00:22:55]     |
[00:22:55]     |
[00:22:55] 307 |                 if log_enabled!(log::Level::Info) {
[00:22:55] 
[00:22:55] 
[00:22:55] error: cannot find macro `log_enabled!` in this scope
[00:22:55]     |
[00:22:55]     |
[00:22:55] 314 |                 if log_enabled!(log::Level::Info) {
[00:22:55] 
[00:22:55] 
[00:22:55] error[E0433]: failed to resolve: could not find `getopts` in `{{root}}`
[00:22:55]     |
[00:22:55] 438 |         matches: &::getopts::Matches,
[00:22:55] 438 |         matches: &::getopts::Matches,
[00:22:55]     |                     ^^^^^^^ could not find `getopts` in `{{root}}`
[00:22:55] 
[00:22:55] error[E0433]: failed to resolve: could not find `errors` in `{{root}}`
[00:22:55]     |
[00:22:55]     |
[00:22:55] 441 |         descriptions: &::errors::registry::Registry,
[00:22:55]     |                          ^^^^^^ could not find `errors` in `{{root}}`
[00:22:55] 
[00:22:55] error[E0433]: failed to resolve: could not find `getopts` in `{{root}}`
[00:22:55]     |
[00:22:55] 454 |         matches: &::getopts::Matches,
[00:22:55] 454 |         matches: &::getopts::Matches,
[00:22:55]     |                     ^^^^^^^ could not find `getopts` in `{{root}}`
[00:22:55] 
[00:22:55] error[E0433]: failed to resolve: could not find `errors` in `{{root}}`
[00:22:55]     |
[00:22:55]     |
[00:22:55] 459 |         descriptions: &::errors::registry::Registry,
[00:22:55]     |                          ^^^^^^ could not find `errors` in `{{root}}`
[00:22:55] 
[00:22:55] error[E0433]: failed to resolve: could not find `getopts` in `{{root}}`
[00:22:55]     |
[00:22:55] 473 |         matches: &::getopts::Matches,
[00:22:55] 473 |         matches: &::getopts::Matches,
[00:22:55]     |                     ^^^^^^^ could not find `getopts` in `{{root}}`
[00:22:55] 
[00:22:55] error[E0433]: failed to resolve: could not find `getopts` in `{{root}}`
[00:22:55]     |
[00:22:55]     |
[00:22:55] 486 |         _: &::getopts::Matches
[00:22:55]     |               ^^^^^^^ could not find `getopts` in `{{root}}`
[00:22:55] 
[00:22:55] error: cannot find macro `warn!` in this scope
[00:22:55]     |
[00:22:55] 815 |                 warn!(
[00:22:55]     |                 ^^^^
[00:22:55] 
[00:22:55] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/driver.rs:122:15
[00:22:56]     |
[00:22:56] 122 |     control: &CompileController,
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/driver.rs:400:34
[00:22:56]     |
[00:22:56]     |
[00:22:56] 400 |     pub provide: Box<dyn Fn(&mut ty::query::Providers) + 'a + sync::Send>,
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/driver.rs:403:41
[00:22:56]     |
[00:22:56]     |
[00:22:56] 403 |     pub provide_extern: Box<dyn Fn(&mut ty::query::Providers) + 'a + sync::Send>,
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/driver.rs:646:15
[00:22:56]     |
[00:22:56]     |
[00:22:56] 646 |     control: &CompileController,
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/driver.rs:726:22
[00:22:56]     |
---
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]     --> src/librustc_driver/driver.rs:1143:40
[00:22:56]      |
[00:22:56] 1143 | pub fn default_provide(providers: &mut ty::query::Providers) {
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]     --> src/librustc_driver/driver.rs:1167:47
[00:22:56]      |
[00:22:56]      |
[00:22:56] 1167 | pub fn default_provide_extern(providers: &mut ty::query::Providers) {
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]     --> src/librustc_driver/driver.rs:1176:15
[00:22:56]      |
[00:22:56]      |
[00:22:56] 1176 |     control: &CompileController,
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/pretty.rs:603:70
[00:22:56]     |
[00:22:56]     |
[00:22:56] 603 |     fn to_one_node_id(self, user_option: &str, sess: &Session, map: &hir_map::Map) -> ast::NodeId {
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]    --> src/librustc_driver/lib.rs:912:43
[00:22:56]     |
[00:22:56]     |
[00:22:56] 912 | pub fn enable_save_analysis(control: &mut CompileController) {
[00:22:56] 
[00:22:56] error: hidden lifetime parameters in types are deprecated
[00:22:56]     --> src/librustc_driver/lib.rs:1520:27
[00:22:56]      |
---
[00:22:56] 
[00:22:56] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:22:56]    --> src/librustc_driver/lib.rs:561:54
[00:22:56]     |
[00:22:56] 561 |         matches.opt_default("pretty", "normal").map(|a| {
[00:22:56]     |                                                      ^ doesn't have a size known at compile-time
[00:22:56]     = help: the trait `std::marker::Sized` is not implemented for `str`
[00:22:56]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:22:56]     = note: all local variables must have a statically known size
[00:22:56]     = help: unsized locals are gated as an unstable feature
[00:22:56]     = help: unsized locals are gated as an unstable feature
[00:22:56] 
[00:22:56] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:22:56]    --> src/librustc_driver/lib.rs:772:16
[00:22:56]     |
[00:22:56] 772 |         if let Some(ref code) = matches.opt_str("explain") {
[00:22:56]     |
[00:22:56]     = help: the trait `std::marker::Sized` is not implemented for `str`
[00:22:56]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:22:56]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:22:56]     = note: required by `std::prelude::v1::Some`
[00:22:56] error: aborting due to 25 previous errors
[00:22:56] 
[00:22:56] Some errors occurred: E0277, E0432, E0433.
[00:22:56] For more information about an error, try `rustc --explain E0277`.
[00:22:56] For more information about an error, try `rustc --explain E0277`.
[00:22:56] error: Could not compile `rustc_driver`.
[00:22:56] warning: build failed, waiting for other jobs to finish...
[00:23:13] error: build failed
[00:23:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:23:13] expected success, got: exit code: 101
[00:23:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:13] Build completed unsuccessfully in 0:18:51
[00:23:14] Makefile:18: recipe for target 'all' failed
[00:23:14] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f4b2718
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 05:47:01 UTC 2019
