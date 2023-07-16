plain
2019-12-23T11:16:02.6607783Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T11:16:02.6804247Z ##[command]git config gc.auto 0
2019-12-23T11:16:02.6872700Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T11:16:02.6929271Z ##[command]git config --get-all http.proxy
2019-12-23T11:16:02.7078445Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67258/merge:refs/remotes/pull/67258/merge
---
2019-12-23T11:23:27.8441896Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-12-23T11:23:28.0649913Z error: cannot find macro `struct_span_err` in this scope
2019-12-23T11:23:28.0651000Z    --> src/librustc_parse/parser/pat.rs:726:9
2019-12-23T11:23:28.0651917Z     |
2019-12-23T11:23:28.0653053Z 726 |         struct_span_err!(self.sess.span_diagnostic, span, E0586, "inclusive range with no end")
2019-12-23T11:23:28.0653456Z 
2019-12-23T11:23:28.2172158Z error: unused import: `super::diagnostics::Error`
2019-12-23T11:23:28.2172574Z  --> src/librustc_parse/parser/expr.rs:1:5
2019-12-23T11:23:28.2172812Z   |
---
2019-12-23T11:23:28.2233564Z 
2019-12-23T11:23:28.4784025Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2019-12-23T11:23:28.4784878Z    --> src/librustc_parse/parser/pat.rs:295:25
2019-12-23T11:23:28.4785112Z     |
2019-12-23T11:23:28.4785438Z 295 |                 if self.is_pat_range_end_start() {
2019-12-23T11:23:28.4786034Z ...
2019-12-23T11:23:28.4786034Z ...
2019-12-23T11:23:28.4786338Z 739 |     fn is_pat_range_end_start(&self, dist: usize) -> bool {
2019-12-23T11:23:28.4786970Z 
2019-12-23T11:23:28.4825262Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-12-23T11:23:28.4825611Z    --> src/librustc_parse/parser/pat.rs:297:26
2019-12-23T11:23:28.4825835Z     |
2019-12-23T11:23:28.4825835Z     |
2019-12-23T11:23:28.4826356Z 297 |                     self.parse_pat_range_to(RangeEnd::Excluded, "..")?
2019-12-23T11:23:28.4827197Z ...
2019-12-23T11:23:28.4827197Z ...
2019-12-23T11:23:28.4827539Z 732 |     fn parse_pat_range_to(&mut self, re: Spanned<RangeEnd>) -> PResult<'a, PatKind> {
2019-12-23T11:23:28.4828283Z 
2019-12-23T11:23:28.4862105Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-12-23T11:23:28.4862392Z    --> src/librustc_parse/parser/pat.rs:306:22
2019-12-23T11:23:28.4862638Z     |
2019-12-23T11:23:28.4862638Z     |
2019-12-23T11:23:28.4863279Z 306 |                 self.parse_pat_range_to(RangeEnd::Included(RangeSyntax::DotDotEq), "..=")?
2019-12-23T11:23:28.4863976Z ...
2019-12-23T11:23:28.4863976Z ...
2019-12-23T11:23:28.4864793Z 732 |     fn parse_pat_range_to(&mut self, re: Spanned<RangeEnd>) -> PResult<'a, PatKind> {
2019-12-23T11:23:28.4865242Z 
2019-12-23T11:23:28.4901986Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-12-23T11:23:28.4902440Z    --> src/librustc_parse/parser/pat.rs:311:22
2019-12-23T11:23:28.4902854Z     |
2019-12-23T11:23:28.4902854Z     |
2019-12-23T11:23:28.4903146Z 311 |                 self.parse_pat_range_to(RangeEnd::Included(RangeSyntax::DotDotDot), "...")?
2019-12-23T11:23:28.4903659Z ...
2019-12-23T11:23:28.4903659Z ...
2019-12-23T11:23:28.4904375Z 732 |     fn parse_pat_range_to(&mut self, re: Spanned<RangeEnd>) -> PResult<'a, PatKind> {
2019-12-23T11:23:28.4904849Z 
2019-12-23T11:23:29.2230740Z error[E0308]: mismatched types
2019-12-23T11:23:29.2231115Z    --> src/librustc_parse/parser/pat.rs:448:25
2019-12-23T11:23:29.2231633Z     |
2019-12-23T11:23:29.2231633Z     |
2019-12-23T11:23:29.2231934Z 443 |     fn ban_pat_range_if_ambiguous(&self, pat: &Pat) {
2019-12-23T11:23:29.2232605Z ...
2019-12-23T11:23:29.2233036Z 448 |             ) => return Ok(()),
2019-12-23T11:23:29.2233865Z     |                         ^^^^^^ expected `()`, found enum `std::result::Result`
2019-12-23T11:23:29.2234164Z     |
2019-12-23T11:23:29.2234164Z     |
2019-12-23T11:23:29.2234780Z     = note: expected unit type `()`
2019-12-23T11:23:29.2235080Z                     found enum `std::result::Result<(), _>`
2019-12-23T11:23:29.2235261Z 
2019-12-23T11:23:29.4031095Z error[E0308]: mismatched types
2019-12-23T11:23:29.4031415Z    --> src/librustc_parse/parser/pat.rs:462:9
2019-12-23T11:23:29.4031604Z     |
2019-12-23T11:23:29.4031883Z 443 |     fn ban_pat_range_if_ambiguous(&self, pat: &Pat) {
2019-12-23T11:23:29.4032433Z ...
2019-12-23T11:23:29.4032675Z 462 |         Err(err)
2019-12-23T11:23:29.4032675Z 462 |         Err(err)
2019-12-23T11:23:29.4032985Z     |         ^^^^^^^^- help: try adding a semicolon: `;`
2019-12-23T11:23:29.4033509Z     |         expected `()`, found enum `std::result::Result`
2019-12-23T11:23:29.4033694Z     |
2019-12-23T11:23:29.4033943Z     = note: expected unit type `()`
2019-12-23T11:23:29.4034651Z                     found enum `std::result::Result<_, rustc_errors::diagnostic_builder::DiagnosticBuilder<'_>>`
2019-12-23T11:23:29.4034651Z                     found enum `std::result::Result<_, rustc_errors::diagnostic_builder::DiagnosticBuilder<'_>>`
2019-12-23T11:23:29.4034713Z 
2019-12-23T11:23:29.4233558Z error[E0599]: no method named `parse_pat_range_end_opt` found for type `&mut parser::Parser<'a>` in the current scope
2019-12-23T11:23:29.4233928Z    --> src/librustc_parse/parser/pat.rs:644:24
2019-12-23T11:23:29.4234133Z     |
2019-12-23T11:23:29.4235213Z 644 |         let end = self.parse_pat_range_end_opt(&begin, form)?;
2019-12-23T11:23:29.4235651Z     |                        ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `parse_pat_range_end`
2019-12-23T11:23:29.4770991Z error[E0308]: mismatched types
2019-12-23T11:23:29.4771321Z    --> src/librustc_parse/parser/pat.rs:645:27
2019-12-23T11:23:29.4771586Z     |
2019-12-23T11:23:29.4771586Z     |
2019-12-23T11:23:29.4771878Z 645 |         Ok(PatKind::Range(begin, end, respan(op_span, end_kind)))
2019-12-23T11:23:29.4772577Z     |                           |
2019-12-23T11:23:29.4772907Z     |                           expected enum `std::option::Option`, found struct `syntax::ptr::P`
2019-12-23T11:23:29.4773217Z     |                           help: try using a variant of the expected enum: `Some(begin)`
2019-12-23T11:23:29.4773440Z     |
2019-12-23T11:23:29.4773440Z     |
2019-12-23T11:23:29.4773913Z     = note: expected enum `std::option::Option<syntax::ptr::P<_>>`
2019-12-23T11:23:29.4774598Z              found struct `syntax::ptr::P<_>`
2019-12-23T11:23:29.4774645Z 
2019-12-23T11:23:29.4836588Z error[E0599]: no method named `parse_pat_range_end_opt` found for type `&mut parser::Parser<'a>` in the current scope
2019-12-23T11:23:29.4836922Z    --> src/librustc_parse/parser/pat.rs:661:24
2019-12-23T11:23:29.4837405Z     |
2019-12-23T11:23:29.4837884Z 661 |         let end = self.parse_pat_range_end_opt(&begin, form)?;
2019-12-23T11:23:29.4838389Z     |                        ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `parse_pat_range_end`
2019-12-23T11:23:29.5370943Z error[E0308]: mismatched types
2019-12-23T11:23:29.5371315Z    --> src/librustc_parse/parser/pat.rs:662:27
2019-12-23T11:23:29.5371574Z     |
2019-12-23T11:23:29.5371574Z     |
2019-12-23T11:23:29.5371867Z 662 |         Ok(PatKind::Range(begin, end, respan(op_span, end_kind)))
2019-12-23T11:23:29.5372477Z     |                           |
2019-12-23T11:23:29.5372795Z     |                           expected enum `std::option::Option`, found struct `syntax::ptr::P`
2019-12-23T11:23:29.5373133Z     |                           help: try using a variant of the expected enum: `Some(begin)`
2019-12-23T11:23:29.5373345Z     |
2019-12-23T11:23:29.5373345Z     |
2019-12-23T11:23:29.5373659Z     = note: expected enum `std::option::Option<syntax::ptr::P<_>>`
2019-12-23T11:23:29.5373910Z              found struct `syntax::ptr::P<_>`
2019-12-23T11:23:29.5373944Z 
2019-12-23T11:23:29.6262926Z error[E0599]: no variant or associated item named `InclusiveRangeWithNoEnd` found for type `parser::diagnostics::Error` in the current scope
2019-12-23T11:23:29.6263640Z     |
2019-12-23T11:23:29.6263855Z 41  | pub enum Error {
2019-12-23T11:23:29.6263855Z 41  | pub enum Error {
2019-12-23T11:23:29.6264315Z     | -------------- variant or associated item `InclusiveRangeWithNoEnd` not found here
2019-12-23T11:23:29.6264969Z ...
2019-12-23T11:23:29.6265318Z 104 |             Error::InclusiveRangeWithNoEnd => {
2019-12-23T11:23:29.6265740Z     |                    ^^^^^^^^^^^^^^^^^^^^^^^ variant or associated item not found in `parser::diagnostics::Error`
2019-12-23T11:23:29.9294610Z error: aborting due to 14 previous errors
2019-12-23T11:23:29.9294735Z 
2019-12-23T11:23:29.9295062Z Some errors have detailed explanations: E0061, E0308, E0599.
2019-12-23T11:23:29.9295377Z For more information about an error, try `rustc --explain E0061`.
2019-12-23T11:23:29.9295377Z For more information about an error, try `rustc --explain E0061`.
2019-12-23T11:23:29.9338206Z error: could not compile `rustc_parse`.
2019-12-23T11:23:29.9338547Z warning: build failed, waiting for other jobs to finish...
2019-12-23T11:23:31.3671171Z error[E0412]: cannot find type `Size` in this scope
2019-12-23T11:23:31.3672596Z   --> src/librustc/ty/util.rs:49:21
2019-12-23T11:23:31.3673073Z    |
2019-12-23T11:23:31.3673575Z 49 | fn signed_min(size: Size) -> i128 {
2019-12-23T11:23:31.3675335Z    |
2019-12-23T11:23:31.3675879Z help: a trait with a similar name exists
2019-12-23T11:23:31.3676374Z    |
2019-12-23T11:23:31.3676374Z    |
2019-12-23T11:23:31.3676919Z 49 | fn signed_min(size: Sized) -> i128 {
2019-12-23T11:23:31.3678181Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T11:23:31.3678676Z    |
2019-12-23T11:23:31.3679176Z 3  | use rustc_target::abi::Size;
2019-12-23T11:23:31.3679603Z    |
2019-12-23T11:23:31.3679603Z    |
2019-12-23T11:23:31.3679836Z 
2019-12-23T11:23:31.3877495Z error[E0412]: cannot find type `Size` in this scope
2019-12-23T11:23:31.3878931Z   --> src/librustc/ty/util.rs:53:21
2019-12-23T11:23:31.3879448Z    |
2019-12-23T11:23:31.3879936Z 53 | fn signed_max(size: Size) -> i128 {
2019-12-23T11:23:31.3880844Z    |
2019-12-23T11:23:31.3881321Z help: a trait with a similar name exists
2019-12-23T11:23:31.3881760Z    |
2019-12-23T11:23:31.3881760Z    |
2019-12-23T11:23:31.3882227Z 53 | fn signed_max(size: Sized) -> i128 {
2019-12-23T11:23:31.3883207Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T11:23:31.3883668Z    |
2019-12-23T11:23:31.3884186Z 3  | use rustc_target::abi::Size;
2019-12-23T11:23:31.3885796Z    |
2019-12-23T11:23:31.3885796Z    |
2019-12-23T11:23:31.3890132Z 
2019-12-23T11:23:31.4086555Z error[E0412]: cannot find type `Size` in this scope
2019-12-23T11:23:31.4091045Z   --> src/librustc/ty/util.rs:57:23
2019-12-23T11:23:31.4091333Z    |
2019-12-23T11:23:31.4091651Z 57 | fn unsigned_max(size: Size) -> u128 {
2019-12-23T11:23:31.4092126Z    |
2019-12-23T11:23:31.4092406Z help: a trait with a similar name exists
2019-12-23T11:23:31.4092614Z    |
2019-12-23T11:23:31.4092614Z    |
2019-12-23T11:23:31.4092882Z 57 | fn unsigned_max(size: Sized) -> u128 {
2019-12-23T11:23:31.4093448Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T11:23:31.4093684Z    |
2019-12-23T11:23:31.4094163Z 3  | use rustc_target::abi::Size;
2019-12-23T11:23:31.4094943Z    |
2019-12-23T11:23:31.4094943Z    |
2019-12-23T11:23:31.4099569Z 
2019-12-23T11:23:31.4293595Z error[E0412]: cannot find type `Size` in this scope
2019-12-23T11:23:31.4293918Z   --> src/librustc/ty/util.rs:61:67
2019-12-23T11:23:31.4294980Z    |
2019-12-23T11:23:31.4295362Z 61 | fn int_size_and_signed<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> (Size, bool) {
2019-12-23T11:23:31.4295919Z    |
2019-12-23T11:23:31.4296187Z help: a trait with a similar name exists
2019-12-23T11:23:31.4296415Z    |
2019-12-23T11:23:31.4296415Z    |
2019-12-23T11:23:31.4296741Z 61 | fn int_size_and_signed<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> (Sized, bool) {
2019-12-23T11:23:31.4297383Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T11:23:31.4297653Z    |
2019-12-23T11:23:31.4298101Z 3  | use rustc_target::abi::Size;
2019-12-23T11:23:31.4298449Z    |
2019-12-23T11:23:31.4298449Z    |
2019-12-23T11:23:31.4302318Z 
2019-12-23T11:23:56.6769128Z error: aborting due to 4 previous errors
2019-12-23T11:23:56.6769303Z 
2019-12-23T11:23:56.6769872Z For more information about this error, try `rustc --explain E0412`.
2019-12-23T11:23:56.7095068Z error: could not compile `rustc`.
2019-12-23T11:23:56.7095817Z 
2019-12-23T11:23:56.7096318Z To learn more, run the command again with --verbose.
2019-12-23T11:23:56.7122403Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T11:23:56.7144081Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T11:23:56.7144443Z Build completed unsuccessfully in 0:04:41
2019-12-23T11:23:56.7205182Z == clock drift check ==
2019-12-23T11:23:56.7221468Z   local time: Mon Dec 23 11:23:56 UTC 2019
2019-12-23T11:23:56.7221468Z   local time: Mon Dec 23 11:23:56 UTC 2019
2019-12-23T11:23:56.7661025Z   network time: Mon, 23 Dec 2019 11:23:56 GMT
2019-12-23T11:23:56.7664459Z == end clock drift check ==
2019-12-23T11:23:57.3470271Z 
2019-12-23T11:23:57.3582460Z ##[error]Bash exited with code '1'.
2019-12-23T11:23:57.3610989Z ##[section]Starting: Checkout
2019-12-23T11:23:57.3612557Z ==============================================================================
2019-12-23T11:23:57.3612644Z Task         : Get sources
2019-12-23T11:23:57.3612690Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
