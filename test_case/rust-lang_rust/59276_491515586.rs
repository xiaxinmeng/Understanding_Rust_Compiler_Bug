plain
travis_time:end:071488f8:start=1557583523216496901,finish=1557583525461942327,duration=2245445426
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:22:30]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:22:30] error[E0433]: failed to resolve: use of undeclared type or module `ConstValue`
[00:22:30]    --> src/librustc_codegen_utils/symbol_names.rs:445:16
[00:22:30]     |
[00:22:30] 445 |         if let ConstValue::Scalar(Scalar::Bits { .. }) = ct.val {
[00:22:30] 
[00:22:30] error[E0433]: failed to resolve: use of undeclared type or module `Scalar`
[00:22:30]    --> src/librustc_codegen_utils/symbol_names.rs:445:35
[00:22:30]     |
[00:22:30]     |
[00:22:30] 445 |         if let ConstValue::Scalar(Scalar::Bits { .. }) = ct.val {
[00:22:30]     |                                   ^^^^^^ use of undeclared type or module `Scalar`
[00:22:30] error[E0308]: mismatched types
[00:22:30]    --> src/librustc_codegen_utils/symbol_names.rs:447:17
[00:22:30]     |
[00:22:30]     |
[00:22:30] 446 |             match ct.ty {
[00:22:30]     |                   ----- this match expression has type `rustc::ty::TyS<'_>`
[00:22:30] 447 |                 ty::Int(_) | ty::Uint(_) => return self.pretty_print_const(ct)?,
[00:22:30]     |                 ^^^^^^^^^^ expected struct `rustc::ty::TyS`, found enum `rustc::ty::TyKind`
[00:22:30]     = note: expected type `rustc::ty::TyS<'_>`
[00:22:30]                found type `rustc::ty::TyKind<'_>`
[00:22:30] 
[00:22:30] error[E0308]: mismatched types
[00:22:30] error[E0308]: mismatched types
[00:22:30]    --> src/librustc_codegen_utils/symbol_names.rs:447:30
[00:22:30]     |
[00:22:30] 446 |             match ct.ty {
[00:22:30]     |                   ----- this match expression has type `rustc::ty::TyS<'_>`
[00:22:30] 447 |                 ty::Int(_) | ty::Uint(_) => return self.pretty_print_const(ct)?,
[00:22:30]     |                              ^^^^^^^^^^^ expected struct `rustc::ty::TyS`, found enum `rustc::ty::TyKind`
[00:22:30]     = note: expected type `rustc::ty::TyS<'_>`
[00:22:30]                found type `rustc::ty::TyKind<'_>`
[00:22:30] 
[00:22:30] error[E0308]: try expression alternatives have incompatible types
[00:22:30] error[E0308]: try expression alternatives have incompatible types
[00:22:30]    --> src/librustc_codegen_utils/symbol_names.rs:447:52
[00:22:30]     |
[00:22:30] 447 |                 ty::Int(_) | ty::Uint(_) => return self.pretty_print_const(ct)?,
[00:22:30]     |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found struct `symbol_names::SymbolPrinter`
[00:22:30]     |
[00:22:30]     = note: expected type `std::result::Result<symbol_names::SymbolPrinter<'_, '_>, std::fmt::Error>`
[00:22:30]                found type `symbol_names::SymbolPrinter<'_, '_>`
[00:22:30] error: aborting due to 5 previous errors
[00:22:30] 
[00:22:30] Some errors occurred: E0308, E0433.
[00:22:30] For more information about an error, try `rustc --explain E0308`.
