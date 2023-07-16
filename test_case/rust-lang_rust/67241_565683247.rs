plain
2019-12-14T04:44:20.7920804Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T04:44:21.7607706Z ##[command]git config gc.auto 0
2019-12-14T04:44:21.7610637Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T04:44:21.7612394Z ##[command]git config --get-all http.proxy
2019-12-14T04:44:21.7614692Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-14T04:51:50.1120013Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-14T04:52:30.7742856Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-14T04:52:32.4842955Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-12-14T04:52:34.0524871Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-14T04:52:34.9277605Z error[E0433]: failed to resolve: use of undeclared type or module `ClosureOutlivesSubject`
2019-12-14T04:52:34.9278462Z     |
2019-12-14T04:52:34.9278462Z     |
2019-12-14T04:52:34.9278856Z 175 |                         subject: ClosureOutlivesSubject::Region(fr_minus),
2019-12-14T04:52:34.9279348Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ClosureOutlivesSubject`
2019-12-14T04:52:35.2517420Z error[E0412]: cannot find type `GenericKind` in this scope
2019-12-14T04:52:35.2517867Z   --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:73:18
2019-12-14T04:52:35.2518136Z    |
2019-12-14T04:52:35.2518524Z 73 |         generic: GenericKind<'tcx>,
2019-12-14T04:52:35.2518524Z 73 |         generic: GenericKind<'tcx>,
2019-12-14T04:52:35.2518923Z    |                  ^^^^^^^^^^^ not found in this scope
2019-12-14T04:52:35.2519195Z    |
2019-12-14T04:52:35.2519521Z help: possible candidate is found in another module, you can import it into scope
2019-12-14T04:52:35.2519821Z    |
2019-12-14T04:52:35.2520166Z 3  | use rustc::infer::region_constraints::GenericKind;
2019-12-14T04:52:35.2520406Z    |
2019-12-14T04:52:35.2520443Z 
2019-12-14T04:52:35.2560028Z error[E0412]: cannot find type `ClosureOutlivesRequirement` in this scope
2019-12-14T04:52:35.2560742Z     |
2019-12-14T04:52:35.2560742Z     |
2019-12-14T04:52:35.2561202Z 137 |         propagated_outlives_requirements: &mut Option<&mut Vec<ClosureOutlivesRequirement<'tcx>>>,
2019-12-14T04:52:35.2561985Z     |
2019-12-14T04:52:35.2562365Z help: possible candidate is found in another module, you can import it into scope
2019-12-14T04:52:35.2562641Z     |
2019-12-14T04:52:35.2562641Z     |
2019-12-14T04:52:35.2563013Z 3   | use rustc::mir::ClosureOutlivesRequirement;
2019-12-14T04:52:35.2563325Z 
2019-12-14T04:52:35.2657098Z error[E0412]: cannot find type `ErrorReported` in this scope
2019-12-14T04:52:35.2657584Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:141:17
2019-12-14T04:52:35.2657839Z     |
2019-12-14T04:52:35.2657839Z     |
2019-12-14T04:52:35.2658169Z 141 |     ) -> Option<ErrorReported> {
2019-12-14T04:52:35.2658955Z     |
2019-12-14T04:52:35.2659305Z help: possible candidates are found in other modules, you can import them into scope
2019-12-14T04:52:35.2659546Z     |
2019-12-14T04:52:35.2659546Z     |
2019-12-14T04:52:35.2659857Z 3   | use rustc::util::common::ErrorReported;
2019-12-14T04:52:35.2660431Z 3   | use rustc_errors::ErrorReported;
2019-12-14T04:52:35.2660672Z     |
2019-12-14T04:52:35.2660727Z 
2019-12-14T04:52:35.2660727Z 
2019-12-14T04:52:35.2661067Z error[E0422]: cannot find struct, variant or union type `ClosureOutlivesRequirement` in this scope
2019-12-14T04:52:35.2661713Z     |
2019-12-14T04:52:35.2661713Z     |
2019-12-14T04:52:35.2662113Z 174 |                     propagated_outlives_requirements.push(ClosureOutlivesRequirement {
2019-12-14T04:52:35.2662895Z     |
2019-12-14T04:52:35.2663249Z help: possible candidate is found in another module, you can import it into scope
2019-12-14T04:52:35.2663542Z     |
2019-12-14T04:52:35.2663542Z     |
2019-12-14T04:52:35.2663893Z 3   | use rustc::mir::ClosureOutlivesRequirement;
2019-12-14T04:52:35.2664228Z 
2019-12-14T04:52:35.2664555Z error[E0425]: cannot find value `ErrorReported` in this scope
2019-12-14T04:52:35.2664907Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:206:14
2019-12-14T04:52:35.2665189Z     |
2019-12-14T04:52:35.2665189Z     |
2019-12-14T04:52:35.2665525Z 206 |         Some(ErrorReported)
2019-12-14T04:52:35.2665927Z     |              ^^^^^^^^^^^^^ not found in this scope
2019-12-14T04:52:35.2666222Z     |
2019-12-14T04:52:35.2667079Z help: possible candidates are found in other modules, you can import them into scope
2019-12-14T04:52:35.2667347Z     |
2019-12-14T04:52:35.2667681Z 3   | use rustc::util::common::ErrorReported;
2019-12-14T04:52:35.2668227Z 3   | use rustc_errors::ErrorReported;
2019-12-14T04:52:35.2668484Z     |
2019-12-14T04:52:35.2668520Z 
2019-12-14T04:52:35.5595289Z error: unused import: `Diagnostic`
2019-12-14T04:52:35.5595289Z error: unused import: `Diagnostic`
2019-12-14T04:52:35.5595617Z   --> src/librustc_mir/borrow_check/region_infer/mod.rs:21:20
2019-12-14T04:52:35.5596038Z    |
2019-12-14T04:52:35.5596915Z 21 | use rustc_errors::{Diagnostic, DiagnosticBuilder};
2019-12-14T04:52:35.5597654Z    |
2019-12-14T04:52:35.5597958Z    = note: `-D unused-imports` implied by `-D warnings`
2019-12-14T04:52:35.5597997Z 
2019-12-14T04:52:37.9295008Z error[E0308]: mismatched types
2019-12-14T04:52:37.9295008Z error[E0308]: mismatched types
2019-12-14T04:52:37.9297127Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:204:19
2019-12-14T04:52:37.9297989Z     |
2019-12-14T04:52:37.9298776Z 204 |         db.buffer(errors_buffer);
2019-12-14T04:52:37.9299722Z     |                   ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:37.9300381Z     |
2019-12-14T04:52:37.9301177Z     = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-14T04:52:37.9301933Z                found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:37.9342311Z error[E0624]: method `universal_upper_bound` is private
2019-12-14T04:52:37.9343145Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:226:36
2019-12-14T04:52:37.9343874Z     |
2019-12-14T04:52:37.9343874Z     |
2019-12-14T04:52:37.9344639Z 226 |             let upper_bound = self.universal_upper_bound(r);
2019-12-14T04:52:37.9345725Z 
2019-12-14T04:52:38.8241930Z error[E0308]: mismatched types
2019-12-14T04:52:38.8242305Z    --> src/librustc_mir/borrow_check/region_infer/mod.rs:915:29
2019-12-14T04:52:38.8242507Z     |
2019-12-14T04:52:38.8242507Z     |
2019-12-14T04:52:38.8242786Z 915 |                     .buffer(errors_buffer);
2019-12-14T04:52:38.8243151Z     |                             ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:38.8243374Z     |
2019-12-14T04:52:38.8243671Z     = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-14T04:52:38.8243933Z                found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:39.2080322Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-12-14T04:52:39.2718197Z error[E0308]: mismatched types
2019-12-14T04:52:39.2718668Z    --> src/librustc_mir/borrow_check/region_infer/mod.rs:926:36
2019-12-14T04:52:39.2718934Z     |
2019-12-14T04:52:39.2718934Z     |
2019-12-14T04:52:39.2719311Z 926 |                   errors_buffer.push(RegionErrorKind::TypeTestDoesNotLiveLongEnough {
2019-12-14T04:52:39.2720344Z 927 | |                     span: type_test_span,
2019-12-14T04:52:39.2720344Z 927 | |                     span: type_test_span,
2019-12-14T04:52:39.2720715Z 928 | |                     generic: type_test.generic_kind,
2019-12-14T04:52:39.2721246Z 929 | |                 });
2019-12-14T04:52:39.2721660Z     | |_________________^ expected (), found enum `borrow_check::diagnostics::region_errors::RegionErrorKind`
2019-12-14T04:52:39.2722238Z     = note: expected type `()`
2019-12-14T04:52:39.2722578Z                found type `borrow_check::diagnostics::region_errors::RegionErrorKind<'_>`
2019-12-14T04:52:39.2727849Z 
2019-12-14T04:52:39.4255505Z error[E0308]: mismatched types
2019-12-14T04:52:39.4255505Z error[E0308]: mismatched types
2019-12-14T04:52:39.4255872Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1374:63
2019-12-14T04:52:39.4256147Z      |
2019-12-14T04:52:39.4256501Z 1374 |         outlives_suggestion.add_suggestion(body, self, infcx, errors_buffer, region_naming);
2019-12-14T04:52:39.4256940Z      |                                                               ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:39.4257313Z      |
2019-12-14T04:52:39.4258131Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-14T04:52:39.4258457Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:39.4258829Z error[E0624]: method `report_or_propagate_universal_region_error` is private
2019-12-14T04:52:39.4259119Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1448:18
2019-12-14T04:52:39.4259388Z      |
2019-12-14T04:52:39.4259724Z 1448 |             self.report_or_propagate_universal_region_error(
2019-12-14T04:52:39.4259724Z 1448 |             self.report_or_propagate_universal_region_error(
2019-12-14T04:52:39.4260061Z      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-14T04:52:39.4260121Z 
2019-12-14T04:52:39.5193578Z error[E0308]: mismatched types
2019-12-14T04:52:39.5195243Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1482:63
2019-12-14T04:52:39.5196000Z      |
2019-12-14T04:52:39.5196791Z 1482 |         outlives_suggestion.add_suggestion(body, self, infcx, errors_buffer, region_naming);
2019-12-14T04:52:39.5198497Z      |                                                               ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:39.5199192Z      |
2019-12-14T04:52:39.5199802Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-14T04:52:39.5200674Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:39.5260883Z error[E0624]: method `report_or_propagate_universal_region_error` is private
2019-12-14T04:52:39.5261621Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1588:14
2019-12-14T04:52:39.5262441Z      |
2019-12-14T04:52:39.5263065Z 1588 |         self.report_or_propagate_universal_region_error(
2019-12-14T04:52:39.5263065Z 1588 |         self.report_or_propagate_universal_region_error(
2019-12-14T04:52:39.5263642Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-14T04:52:39.5263852Z 
2019-12-14T04:52:39.6191515Z error[E0308]: mismatched types
2019-12-14T04:52:39.6192574Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1697:21
2019-12-14T04:52:39.6193149Z      |
2019-12-14T04:52:39.6193703Z 1697 |             .buffer(errors_buffer);
2019-12-14T04:52:39.6194419Z      |                     ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:39.6194933Z      |
2019-12-14T04:52:39.6195581Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-14T04:52:39.6196147Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-14T04:52:40.3562679Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-12-14T04:52:41.5389445Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-12-14T04:52:42.1231724Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-12-14T04:52:42.3313944Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
---
2019-12-14T04:52:45.1079103Z For more information about an error, try `rustc --explain E0308`.
2019-12-14T04:52:45.1841980Z error: could not compile `rustc_mir`.
2019-12-14T04:52:45.1842327Z warning: build failed, waiting for other jobs to finish...
2019-12-14T04:52:45.4149934Z error: build failed
2019-12-14T04:52:45.4185944Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-14T04:52:45.4198482Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-14T04:52:45.4199049Z Build completed unsuccessfully in 0:05:02
2019-12-14T04:52:45.4261835Z == clock drift check ==
2019-12-14T04:52:45.4279188Z   local time: Sat Dec 14 04:52:45 UTC 2019
2019-12-14T04:52:45.4279188Z   local time: Sat Dec 14 04:52:45 UTC 2019
2019-12-14T04:52:45.7102655Z   network time: Sat, 14 Dec 2019 04:52:45 GMT
2019-12-14T04:52:45.7108060Z == end clock drift check ==
2019-12-14T04:52:46.9444834Z 
2019-12-14T04:52:46.9560036Z ##[error]Bash exited with code '1'.
2019-12-14T04:52:46.9593170Z ##[section]Starting: Checkout
2019-12-14T04:52:46.9595222Z ==============================================================================
2019-12-14T04:52:46.9595285Z Task         : Get sources
2019-12-14T04:52:46.9595516Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
