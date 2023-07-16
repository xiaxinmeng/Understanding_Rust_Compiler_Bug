plain
travis_time:end:22b71300:start=1549237443662756528,finish=1549237536394094041,duration=92731337513
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:16:54]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:18:35]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:19:33]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:19:33]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:19:33] error: incorrect close delimiter: `]`
[00:19:33]     |
[00:19:33] 10  | impl<'a> Resolver<'a> {
[00:19:33]     |                       - un-closed delimiter
[00:19:33] ...
[00:19:33] ...
[00:19:33] 408 | ]
[00:19:33]     | ^ incorrect close delimiter
[00:19:33]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:19:33]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:19:33] error[E0432]: unresolved import `error_reporting::smart_resolve_report_errors`
[00:19:33]    |
[00:19:33]    |
[00:19:33] 76 | use error_reporting::{smart_resolve_report_errors};
[00:19:33]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `smart_resolve_report_errors` in `error_reporting`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]   --> src/librustc_resolve/error_reporting.rs:23:45
[00:19:33]    |
[00:19:33]    |
[00:19:33] 23 |         let is_enum_variant = &|def| if let Def::Variant(..) = def { true } else { false };
[00:19:33]    |                                             ^^^ use of undeclared type or module `Def`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `ModuleOrUniformRoot`
[00:19:33]    |
[00:19:33]    |
[00:19:33] 45 |                     PathResult::Module(ModuleOrUniformRoot::Module(module)) =>
[00:19:33]    |                                        ^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ModuleOrUniformRoot`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `DiagnosticId`
[00:19:33]   --> src/librustc_resolve/error_reporting.rs:56:20
[00:19:33]    |
[00:19:33]    |
[00:19:33] 56 |         let code = DiagnosticId::Error(code.into());
[00:19:33]    |                    ^^^^^^^^^^^^ use of undeclared type or module `DiagnosticId`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]   --> src/librustc_resolve/error_reporting.rs:66:17
[00:19:33]    |
[00:19:33]    |
[00:19:33] 66 |                 Applicability::MaybeIncorrect,
[00:19:33]    |                 ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `DiagnosticId`
[00:19:33]   --> src/librustc_resolve/error_reporting.rs:73:22
[00:19:33]    |
[00:19:33]    |
[00:19:33] 73 |             err.code(DiagnosticId::Error("E0411".into()));
[00:19:33]    |                      ^^^^^^^^^^^^ use of undeclared type or module `DiagnosticId`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `DiagnosticId`
[00:19:33]   --> src/librustc_resolve/error_reporting.rs:82:22
[00:19:33]    |
[00:19:33]    |
[00:19:33] 82 |             err.code(DiagnosticId::Error("E0424".into()));
[00:19:33]    |                      ^^^^^^^^^^^^ use of undeclared type or module `DiagnosticId`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]    |
[00:19:33]    |
[00:19:33] 84 |                 PathSource::Pat => {
[00:19:33]    |                 ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:101:49
[00:19:33]     |
[00:19:33]     |
[00:19:33] 101 |         if candidates.is_empty() && is_expected(Def::Enum(DefId::local(CRATE_DEF_INDEX))) {
[00:19:33]     |                                                 ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `DefId`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:101:59
[00:19:33]     |
[00:19:33]     |
[00:19:33] 101 |         if candidates.is_empty() && is_expected(Def::Enum(DefId::local(CRATE_DEF_INDEX))) {
[00:19:33]     |                                                           ^^^^^ use of undeclared type or module `DefId`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:141:21
[00:19:33]     |
[00:19:33] 141 |                     Applicability::MachineApplicable,
[00:19:33] 141 |                     Applicability::MachineApplicable,
[00:19:33]     |                     ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `AssocSuggestion`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 149 |                     AssocSuggestion::Field => {
[00:19:33]     |                     ^^^^^^^^^^^^^^^ use of undeclared type or module `AssocSuggestion`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:154:29
[00:19:33]     |
[00:19:33] 154 |                             Applicability::MachineApplicable,
[00:19:33] 154 |                             Applicability::MachineApplicable,
[00:19:33]     |                             ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `AssocSuggestion`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 162 |                     AssocSuggestion::MethodWithSelf if self_is_available => {
[00:19:33]     |                     ^^^^^^^^^^^^^^^ use of undeclared type or module `AssocSuggestion`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:167:29
[00:19:33]     |
[00:19:33] 167 |                             Applicability::MachineApplicable,
[00:19:33] 167 |                             Applicability::MachineApplicable,
[00:19:33]     |                             ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `AssocSuggestion`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 170 |                     AssocSuggestion::MethodWithSelf | AssocSuggestion::AssocItem => {
[00:19:33]     |                     ^^^^^^^^^^^^^^^ use of undeclared type or module `AssocSuggestion`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `AssocSuggestion`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 170 |                     AssocSuggestion::MethodWithSelf | AssocSuggestion::AssocItem => {
[00:19:33]     |                                                       ^^^^^^^^^^^^^^^ use of undeclared type or module `AssocSuggestion`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:175:29
[00:19:33]     |
[00:19:33] 175 |                             Applicability::MachineApplicable,
[00:19:33] 175 |                             Applicability::MachineApplicable,
[00:19:33]     |                             ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:196:17
[00:19:33]     |
[00:19:33] 196 |                 Applicability::MaybeIncorrect,
[00:19:33]     |                 ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:238:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 238 |             (Def::Macro(..), _) => {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:243:21
[00:19:33]     |
[00:19:33]     |
[00:19:33] 243 |                     Applicability::MaybeIncorrect,
[00:19:33]     |                     ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:246:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 246 |             (Def::TyAlias(..), PathSource::Trait(_)) => {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 246 |             (Def::TyAlias(..), PathSource::Trait(_)) => {
[00:19:33]     |                                ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `nightly_options`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 248 |                 if nightly_options::is_nightly_build() {
[00:19:33]     |                    ^^^^^^^^^^^^^^^ use of undeclared type or module `nightly_options`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:252:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 252 |             (Def::Mod(..), PathSource::Expr(Some(parent))) => match parent.node {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 252 |             (Def::Mod(..), PathSource::Expr(Some(parent))) => match parent.node {
[00:19:33]     |                            ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `ExprKind`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:253:17
[00:19:33]     |
[00:19:33] 253 |                 ExprKind::Field(_, ident) => {
[00:19:33] 253 |                 ExprKind::Field(_, ident) => {
[00:19:33]     |                 ^^^^^^^^ use of undeclared type or module `ExprKind`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:258:25
[00:19:33]     |
[00:19:33] 258 |                         Applicability::MaybeIncorrect,
[00:19:33]     |                         ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `ExprKind`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:261:17
[00:19:33]     |
[00:19:33] 261 |                 ExprKind::MethodCall(ref segment, ..) => {
[00:19:33] 261 |                 ExprKind::MethodCall(ref segment, ..) => {
[00:19:33]     |                 ^^^^^^^^ use of undeclared type or module `ExprKind`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:267:25
[00:19:33]     |
[00:19:33] 267 |                         Applicability::MaybeIncorrect,
[00:19:33]     |                         ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:272:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 272 |             (Def::Enum(..), PathSource::TupleStruct)
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 272 |             (Def::Enum(..), PathSource::TupleStruct)
[00:19:33]     |                             ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:273:20
[00:19:33]     |
[00:19:33]     |
[00:19:33] 273 |                 | (Def::Enum(..), PathSource::Expr(..))  => {
[00:19:33]     |                    ^^^ use of undeclared type or module `Def`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 273 |                 | (Def::Enum(..), PathSource::Expr(..))  => {
[00:19:33]     |                                   ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:286:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 286 |             (Def::Struct(def_id), _) if ns == ValueNS => {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 342 |                         PathSource::Expr(Some(parent)) => {
[00:19:33]     |                         ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `ExprKind`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:344:33
[00:19:33]     |
[00:19:33]     |
[00:19:33] 344 |                                 ExprKind::MethodCall(ref path_assignment, _)  => {
[00:19:33]     |                                 ^^^^^^^^ use of undeclared type or module `ExprKind`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:352:41
[00:19:33]     |
[00:19:33]     |
[00:19:33] 352 |                                         Applicability::MaybeIncorrect
[00:19:33]     |                                         ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] 
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `PathSource`
[00:19:33]     |
[00:19:33]     |
[00:19:33] 364 |                         PathSource::Expr(None) if followed_by_brace == true => {
[00:19:33]     |                         ^^^^^^^^^^ use of undeclared type or module `PathSource`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Applicability`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:370:37
[00:19:33]     |
[00:19:33]     |
[00:19:33] 370 |                                     Applicability::MaybeIncorrect,
[00:19:33]     |                                     ^^^^^^^^^^^^^ use of undeclared type or module `Applicability`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:390:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 390 |             (Def::Union(..), _) |
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:391:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 391 |             (Def::Variant(..), _) |
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:392:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 392 |             (Def::VariantCtor(_, CtorKind::Fictive), _) if ns == ValueNS => {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `CtorKind`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:392:34
[00:19:33]     |
[00:19:33]     |
[00:19:33] 392 |             (Def::VariantCtor(_, CtorKind::Fictive), _) if ns == ValueNS => {
[00:19:33]     |                                  ^^^^^^^^ use of undeclared type or module `CtorKind`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:396:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 396 |             (Def::SelfTy(..), _) if ns == ValueNS => {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:401:14
[00:19:33]     |
[00:19:33]     |
[00:19:33] 401 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:19:33]     |              ^^^ use of undeclared type or module `Def`
[00:19:33] error[E0433]: failed to resolve: use of undeclared type or module `Def`
[00:19:33]    --> src/librustc_resolve/error_reporting.rs:401:37
[00:19:33]     |
[00:19:33]     |
[00:19:33] 401 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:19:33]     |                                     ^^^ use of undeclared type or module `Def`
[00:19:34] error[E0412]: cannot find type `Resolver` in this scope
[00:19:34]   --> src/librustc_resolve/error_reporting.rs:10:10
[00:19:34]    |
[00:19:34] 10 | impl<'a> Resolver<'a> {
---
[00:19:34]    |
[00:19:34] 1  | use syntax::ext::base::Resolver;
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0412]: cannot find type `PathSource` in this scope
[00:19:34]    |
[00:19:34] 17 |         source: PathSource,
[00:19:34]    |                 ^^^^^^^^^^ not found in this scope
[00:19:34] help: possible candidate is found in another module, you can import it into scope
---
[00:19:34]    |
[00:19:34] 1  | use rustc::hir::def::Def;
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0412]: cannot find type `DiagnosticBuilder` in this scope
[00:19:34]    |
[00:19:34]    |
[00:19:34] 19 |     ) -> (DiagnosticBuilder<'a>, Vec<ImportSuggestion>) {
[00:19:34] help: possible candidates are found in other modules, you can import them into scope
[00:19:34]    |
[00:19:34] 1  | use errors::DiagnosticBuilder;
[00:19:34]    |
[00:19:34]    |
[00:19:34] 1  | use syntax::diagnostics::plugin::DiagnosticBuilder;
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0412]: cannot find type `ImportSuggestion` in this scope
[00:19:34]    |
[00:19:34]    |
[00:19:34] 19 |     ) -> (DiagnosticBuilder<'a>, Vec<ImportSuggestion>) {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]    |
[00:19:34] 1  | use ImportSuggestion;
[00:19:34]    |
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0425]: cannot find value `TypeNS` in this scope
[00:19:34]   --> src/librustc_resolve/error_reporting.rs:43:36
[00:19:34]    |
[00:19:34] 43 |                     mod_path, Some(TypeNS), false, span, CrateLint::No
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]    |
[00:19:34] 1  | use rustc::hir::def::Namespace::TypeNS;
[00:19:34]    |
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0425]: cannot find function `is_self_type` in this scope
[00:19:34]   --> src/librustc_resolve/error_reporting.rs:71:12
[00:19:34]    |
[00:19:34] 71 |         if is_self_type(path, ns) {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]    |
[00:19:34] 1  | use is_self_type;
[00:19:34]    |
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0425]: cannot find function `is_self_value` in this scope
[00:19:34]   --> src/librustc_resolve/error_reporting.rs:78:12
[00:19:34]    |
[00:19:34] 78 |         if is_self_value(path, ns) {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]    |
[00:19:34] 1  | use is_self_value;
[00:19:34]    |
[00:19:34]    |
[00:19:34] 
[00:19:34] error[E0425]: cannot find value `CRATE_DEF_INDEX` in this scope
[00:19:34]     |
[00:19:34]     |
[00:19:34] 101 |         if candidates.is_empty() && is_expected(Def::Enum(DefId::local(CRATE_DEF_INDEX))) {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use rustc::hir::def_id::CRATE_DEF_INDEX;
[00:19:34]     |
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0425]: cannot find function `import_candidate_to_enum_paths` in this scope
[00:19:34]     |
[00:19:34]     |
[00:19:34] 106 |                     import_candidate_to_enum_paths(&suggestion)
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use import_candidate_to_enum_paths;
[00:19:34]     |
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0412]: cannot find type `DiagnosticBuilder` in this scope
[00:19:34]     |
[00:19:34] 227 |         err: &mut DiagnosticBuilder<'a>,
[00:19:34]     |                   ^^^^^^^^^^^^^^^^^ not found in this scope
[00:19:34] help: possible candidates are found in other modules, you can import them into scope
[00:19:34] help: possible candidates are found in other modules, you can import them into scope
[00:19:34]     |
[00:19:34] 1   | use errors::DiagnosticBuilder;
[00:19:34]     |
[00:19:34] 1   | use syntax::diagnostics::plugin::DiagnosticBuilder;
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0412]: cannot find type `PathSource` in this scope
[00:19:34]     |
[00:19:34] 229 |         source: PathSource,
[00:19:34]     |                 ^^^^^^^^^^ not found in this scope
[00:19:34] help: possible candidate is found in another module, you can import it into scope
---
[00:19:34] 
[00:19:34] error[E0425]: cannot find function `path_names_to_string` in this scope
[00:19:34]    --> src/librustc_resolve/error_reporting.rs:278:47
[00:19:34]     |
[00:19:34] 278 |                             .map(|suggestion| path_names_to_string(suggestion))
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use path_names_to_string;
[00:19:34]     |
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0425]: cannot find value `ValueNS` in this scope
[00:19:34]     |
[00:19:34]     |
[00:19:34] 286 |             (Def::Struct(def_id), _) if ns == ValueNS => {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:19:34]     |
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0425]: cannot find value `ValueNS` in this scope
[00:19:34]     |
[00:19:34]     |
[00:19:34] 392 |             (Def::VariantCtor(_, CtorKind::Fictive), _) if ns == ValueNS => {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:19:34]     |
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0425]: cannot find value `ValueNS` in this scope
[00:19:34]     |
[00:19:34]     |
[00:19:34] 396 |             (Def::SelfTy(..), _) if ns == ValueNS => {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:19:34]     |
[00:19:34]     |
[00:19:34] 
[00:19:34] error[E0425]: cannot find value `ValueNS` in this scope
[00:19:34]     |
[00:19:34]     |
[00:19:34] 401 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:19:34] help: possible candidate is found in another module, you can import it into scope
[00:19:34]     |
[00:19:34] 1   | use rustc::hir::def::Namespace::ValueNS;
[00:19:34]     |
---
[00:19:34] 
[00:19:34] error: unused import: `smart_resolve_report_errors`
[00:19:34]   --> src/librustc_resolve/lib.rs:76:23
[00:19:34]    |
[00:19:34] 76 | use error_reporting::{smart_resolve_report_errors};
[00:19:34] 
[00:19:34] 
[00:19:34] error[E0599]: no method named `smart_resolve_report_errors` found for type `&mut Resolver<'a>` in the current scope
[00:19:34]      |
[00:19:34]      |
[00:19:34] 3139 |             let (err, candidates) = this.smart_resolve_report_errors(path, span, source, def);
[00:19:34] 
[00:19:35] error: aborting due to 69 previous errors
[00:19:35] 
[00:19:35] Some errors occurred: E0412, E0425, E0432, E0433, E0599.
---
[00:20:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:20:02] expected success, got: exit code: 101
[00:20:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:02] Build completed unsuccessfully in 0:15:59
[00:20:02] make: *** [all] Error 1
[00:20:02] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1faec50f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 00:05:49 UTC 2019
---
travis_time:end:07771f60:start=1549238750644650734,finish=1549238750649485946,duration=4835212
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03409ad4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04772d00
travis_time:start:04772d00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cf8d860
$ dmesg | grep -i kill
