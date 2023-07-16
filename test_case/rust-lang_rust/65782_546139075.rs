plain
2019-10-24T23:04:12.7139300Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T23:04:12.7344310Z ##[command]git config gc.auto 0
2019-10-24T23:04:13.5429017Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T23:04:13.5432628Z ##[command]git config --get-all http.proxy
2019-10-24T23:04:13.5435532Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65782/merge:refs/remotes/pull/65782/merge
---
2019-10-24T23:14:58.4722747Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-24T23:15:17.4394458Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-24T23:15:29.5610174Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-24T23:16:44.9134666Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-24T23:17:05.4211463Z error[E0004]: non-exhaustive patterns: `&Coercion { .. }` not covered
2019-10-24T23:17:05.4211833Z    --> src/librustc/macros.rs:393:53
2019-10-24T23:17:05.4212125Z     |
2019-10-24T23:17:05.4212440Z 382 | / macro_rules! EnumTypeFoldableImpl {
2019-10-24T23:17:05.4212776Z 383 | |     (impl<$($p:tt),*> TypeFoldable<$tcx:tt> for $s:path {
2019-10-24T23:17:05.4213100Z 384 | |         $($variants:tt)*
2019-10-24T23:17:05.4213677Z 385 | |     } $(where $($wc:tt)*)*) => {
2019-10-24T23:17:05.4213990Z ...   |
2019-10-24T23:17:05.4214379Z 393 | |                 EnumTypeFoldableImpl!(@FoldVariants(self, folder) input($($variants)*) output())
2019-10-24T23:17:05.4214942Z     | |                                                     ^^^^ pattern `&Coercion { .. }` not covered
2019-10-24T23:17:05.4215499Z 510 | |     };
2019-10-24T23:17:05.4215789Z 511 | | }
2019-10-24T23:17:05.4215789Z 511 | | }
2019-10-24T23:17:05.4216115Z     | |_- in this expansion of `EnumTypeFoldableImpl!`
2019-10-24T23:17:05.4216608Z    ::: src/librustc/traits/mod.rs:175:1
2019-10-24T23:17:05.4216826Z     |
2019-10-24T23:17:05.4216826Z     |
2019-10-24T23:17:05.4217140Z 175 | / pub enum ObligationCauseCode<'tcx> {
2019-10-24T23:17:05.4217491Z 176 | |     /// Not well classified or should be obvious from the span.
2019-10-24T23:17:05.4217793Z 177 | |     MiscObligation,
2019-10-24T23:17:05.4218315Z ...   |
2019-10-24T23:17:05.4218315Z ...   |
2019-10-24T23:17:05.4218648Z 205 | |     Coercion { source: Ty<'tcx>, target: Ty<'tcx> },
2019-10-24T23:17:05.4218972Z     | |     -------- not covered
2019-10-24T23:17:05.4220179Z 286 | |     TrivialBound,
2019-10-24T23:17:05.4221649Z 287 | | }
2019-10-24T23:17:05.4221649Z 287 | | }
2019-10-24T23:17:05.4222027Z     | |_- `traits::ObligationCauseCode<'tcx>` defined here
2019-10-24T23:17:05.4222240Z ...
2019-10-24T23:17:05.4222563Z 293 | / EnumTypeFoldableImpl! {
2019-10-24T23:17:05.4222910Z 294 | |     impl<'tcx> TypeFoldable<'tcx> for ObligationCauseCode<'tcx> {
2019-10-24T23:17:05.4224008Z 295 | |         (ObligationCauseCode::MiscObligation),
2019-10-24T23:17:05.4224486Z 296 | |         (ObligationCauseCode::SliceOrArrayElem),
2019-10-24T23:17:05.4225011Z 339 | |     }
2019-10-24T23:17:05.4225306Z 340 | | }
2019-10-24T23:17:05.4225779Z     | |_- in this macro invocation
2019-10-24T23:17:05.4226055Z     |
2019-10-24T23:17:05.4226055Z     |
2019-10-24T23:17:05.4226407Z     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
2019-10-24T23:17:05.4226547Z 
2019-10-24T23:17:06.3775371Z error[E0004]: non-exhaustive patterns: `&Coercion { .. }` not covered
2019-10-24T23:17:06.3775733Z    --> src/librustc/macros.rs:400:54
2019-10-24T23:17:06.3775959Z     |
2019-10-24T23:17:06.3776284Z 382 | / macro_rules! EnumTypeFoldableImpl {
2019-10-24T23:17:06.3776623Z 383 | |     (impl<$($p:tt),*> TypeFoldable<$tcx:tt> for $s:path {
2019-10-24T23:17:06.3776955Z 384 | |         $($variants:tt)*
2019-10-24T23:17:06.3777295Z 385 | |     } $(where $($wc:tt)*)*) => {
2019-10-24T23:17:06.3777539Z ...   |
2019-10-24T23:17:06.3777918Z 400 | |                 EnumTypeFoldableImpl!(@VisitVariants(self, visitor) input($($variants)*) output())
2019-10-24T23:17:06.3778333Z     | |                                                      ^^^^ pattern `&Coercion { .. }` not covered
2019-10-24T23:17:06.3778896Z 510 | |     };
2019-10-24T23:17:06.3779188Z 511 | | }
2019-10-24T23:17:06.3779188Z 511 | | }
2019-10-24T23:17:06.3779506Z     | |_- in this expansion of `EnumTypeFoldableImpl!`
2019-10-24T23:17:06.3780397Z    ::: src/librustc/traits/mod.rs:175:1
2019-10-24T23:17:06.3780630Z     |
2019-10-24T23:17:06.3780630Z     |
2019-10-24T23:17:06.3780940Z 175 | / pub enum ObligationCauseCode<'tcx> {
2019-10-24T23:17:06.3781274Z 176 | |     /// Not well classified or should be obvious from the span.
2019-10-24T23:17:06.3781608Z 177 | |     MiscObligation,
2019-10-24T23:17:06.3782113Z ...   |
2019-10-24T23:17:06.3782113Z ...   |
2019-10-24T23:17:06.3782455Z 205 | |     Coercion { source: Ty<'tcx>, target: Ty<'tcx> },
2019-10-24T23:17:06.3782792Z     | |     -------- not covered
2019-10-24T23:17:06.3783350Z 286 | |     TrivialBound,
2019-10-24T23:17:06.3783633Z 287 | | }
2019-10-24T23:17:06.3783633Z 287 | | }
2019-10-24T23:17:06.3784329Z     | |_- `traits::ObligationCauseCode<'tcx>` defined here
2019-10-24T23:17:06.3784615Z ...
2019-10-24T23:17:06.3784906Z 293 | / EnumTypeFoldableImpl! {
2019-10-24T23:17:06.3785422Z 294 | |     impl<'tcx> TypeFoldable<'tcx> for ObligationCauseCode<'tcx> {
2019-10-24T23:17:06.3785750Z 295 | |         (ObligationCauseCode::MiscObligation),
2019-10-24T23:17:06.3786074Z 296 | |         (ObligationCauseCode::SliceOrArrayElem),
2019-10-24T23:17:06.3786630Z 339 | |     }
2019-10-24T23:17:06.3786907Z 340 | | }
2019-10-24T23:17:06.3787220Z     | |_- in this macro invocation
2019-10-24T23:17:06.3787449Z     |
---
2019-10-24T23:17:29.2891717Z   local time: Thu Oct 24 23:17:29 UTC 2019
2019-10-24T23:17:29.9043523Z   network time: Thu, 24 Oct 2019 23:17:29 GMT
2019-10-24T23:17:29.9046432Z == end clock drift check ==
2019-10-24T23:17:30.5727153Z 
2019-10-24T23:17:30.5840717Z ##[error]Bash exited with code '1'.
2019-10-24T23:17:30.5873072Z ##[section]Starting: Checkout
2019-10-24T23:17:30.5874986Z ==============================================================================
2019-10-24T23:17:30.5875060Z Task         : Get sources
2019-10-24T23:17:30.5875123Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
