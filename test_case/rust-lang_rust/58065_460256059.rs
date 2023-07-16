plain
travis_time:end:1f636d93:start=1549287106667753967,finish=1549287108750351301,duration=2082597334
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:17:11]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:17:18]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:17:18]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:17:18]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:17:19] error[E0432]: unresolved import `error_reporting::smart_resolve_report_errors`
[00:17:19]    |
[00:17:19]    |
[00:17:19] 76 | use error_reporting::{smart_resolve_report_errors};
[00:17:19]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `smart_resolve_report_errors` in `error_reporting`
[00:17:19] error[E0425]: cannot find value `TypeNS` in this scope
[00:17:19]   --> src/librustc_resolve/error_reporting.rs:50:36
[00:17:19]    |
[00:17:19]    |
[00:17:19] 50 |                     mod_path, Some(TypeNS), false, span, CrateLint::No
[00:17:19] help: possible candidate is found in another module, you can import it into scope
[00:17:19]    |
[00:17:19] 1  | use rustc::hir::def::Namespace::TypeNS;
[00:17:19]    |
[00:17:19]    |
[00:17:19] 
[00:17:19] error[E0425]: cannot find value `ValueNS` in this scope
[00:17:19]     |
[00:17:19]     |
[00:17:19] 293 |             (Def::Struct(def_id), _) if ns == ValueNS => {
[00:17:19] help: possible candidate is found in another module, you can import it into scope
[00:17:19]     |
[00:17:19] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:17:19]     |
[00:17:19]     |
[00:17:19] 
[00:17:19] error[E0425]: cannot find value `ValueNS` in this scope
[00:17:19]     |
[00:17:19]     |
[00:17:19] 399 |             (Def::VariantCtor(_, CtorKind::Fictive), _) if ns == ValueNS => {
[00:17:19] help: possible candidate is found in another module, you can import it into scope
[00:17:19]     |
[00:17:19] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:17:19]     |
[00:17:19]     |
[00:17:19] 
[00:17:19] error[E0425]: cannot find value `ValueNS` in this scope
[00:17:19]     |
[00:17:19]     |
[00:17:19] 403 |             (Def::SelfTy(..), _) if ns == ValueNS => {
[00:17:19] help: possible candidate is found in another module, you can import it into scope
[00:17:19]     |
[00:17:19] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:17:19]     |
[00:17:19]     |
[00:17:19] 
[00:17:19] error[E0425]: cannot find value `ValueNS` in this scope
[00:17:19]     |
[00:17:19]     |
[00:17:19] 408 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:17:19] help: possible candidate is found in another module, you can import it into scope
[00:17:19]     |
[00:17:19] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:17:19]     |
---
[00:17:19] 
[00:17:19] error: unused import: `smart_resolve_report_errors`
[00:17:19]   --> src/librustc_resolve/lib.rs:76:23
[00:17:19]    |
[00:17:19] 76 | use error_reporting::{smart_resolve_report_errors};
[00:17:19] 
[00:17:20] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:17:20]   --> src/librustc_resolve/error_reporting.rs:37:14
[00:17:20]    |
[00:17:20]    |
[00:17:20] 37 |         let (base_msg, fallback_label, base_span) = if let Some(def) = def {
[00:17:20]    |
[00:17:20]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:17:20]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:17:20]    = note: all local variables must have a statically known size
[00:17:20]    = note: all local variables must have a statically known size
[00:17:20]    = help: unsized locals are gated as an unstable feature
[00:17:20] 
[00:17:20] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:17:20]   --> src/librustc_resolve/error_reporting.rs:37:24
[00:17:20]    |
[00:17:20] 37 |         let (base_msg, fallback_label, base_span) = if let Some(def) = def {
[00:17:20]    |
[00:17:20]    = help: the trait `std::marker::Sized` is not implemented for `str`
[00:17:20]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:17:20]    = note: all local variables must have a statically known size
[00:17:20]    = note: all local variables must have a statically known size
[00:17:20]    = help: unsized locals are gated as an unstable feature
[00:17:20] 
[00:17:20] error[E0277]: the size for values of type `str` cannot be known at compilation time
[00:17:20]    --> src/librustc_resolve/error_reporting.rs:223:17
[00:17:20]     |
[00:17:20] 223 |             err.span_label(base_span, fallback_label);
[00:17:20]     |
[00:17:20]     = help: the trait `std::marker::Sized` is not implemented for `str`
[00:17:20]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:17:20] 
[00:17:20] 
[00:17:20] error[E0277]: the trait bound `std::string::String: std::convert::From<str>` is not satisfied
[00:17:20]    --> src/librustc_resolve/error_reporting.rs:223:17
[00:17:20]     |
[00:17:20] 223 |             err.span_label(base_span, fallback_label);
[00:17:20]     |                 ^^^^^^^^^^ the trait `std::convert::From<str>` is not implemented for `std::string::String`
[00:17:20]     = help: the following implementations were found:
[00:17:20]     = help: the following implementations were found:
[00:17:20]               <std::string::String as std::convert::From<&'a str>>
[00:17:20]               <std::string::String as std::convert::From<std::borrow::Cow<'a, str>>>
[00:17:20]               <std::string::String as std::convert::From<std::boxed::Box<str>>>
[00:17:20]               <std::string::String as std::convert::From<syntax::symbol::InternedString>>
[00:17:20]     = note: required because of the requirements on the impl of `std::convert::Into<std::string::String>` for `str`
[00:17:21] error: aborting due to 12 previous errors
[00:17:21] 
[00:17:21] Some errors occurred: E0277, E0425, E0432.
[00:17:21] For more information about an error, try `rustc --explain E0277`.
[00:17:21] For more information about an error, try `rustc --explain E0277`.
[00:17:21] error: Could not compile `rustc_resolve`.
[00:17:21] warning: build failed, waiting for other jobs to finish...
[00:18:11] error: build failed
[00:18:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:11] expected success, got: exit code: 101
[00:18:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:11] Build completed unsuccessfully in 0:14:24
[00:18:11] make: *** [all] Error 1
[00:18:11] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07813af6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 13:50:09 UTC 2019
