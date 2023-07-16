plain
2019-12-13T00:11:23.3768783Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-13T00:11:23.4033458Z ##[command]git config gc.auto 0
2019-12-13T00:11:23.4121234Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-13T00:11:23.4191153Z ##[command]git config --get-all http.proxy
2019-12-13T00:11:23.4341400Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-13T00:19:12.2650662Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-13T00:19:14.0010951Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-12-13T00:19:15.4923886Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-12-13T00:19:16.5840645Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-13T00:19:17.5597291Z error[E0433]: failed to resolve: use of undeclared type or module `ClosureOutlivesSubject`
2019-12-13T00:19:17.5598063Z     |
2019-12-13T00:19:17.5598063Z     |
2019-12-13T00:19:17.5603116Z 165 |                         subject: ClosureOutlivesSubject::Region(fr_minus),
2019-12-13T00:19:17.5603876Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ClosureOutlivesSubject`
2019-12-13T00:19:17.5603940Z 
2019-12-13T00:19:17.9217644Z error[E0412]: cannot find type `ClosureOutlivesRequirement` in this scope
2019-12-13T00:19:17.9218358Z     |
2019-12-13T00:19:17.9218358Z     |
2019-12-13T00:19:17.9219275Z 127 |         propagated_outlives_requirements: &mut Option<&mut Vec<ClosureOutlivesRequirement<'tcx>>>,
2019-12-13T00:19:17.9220066Z     |
2019-12-13T00:19:17.9220345Z help: possible candidate is found in another module, you can import it into scope
2019-12-13T00:19:17.9220563Z     |
2019-12-13T00:19:17.9220563Z     |
2019-12-13T00:19:17.9220881Z 3   | use rustc::mir::ClosureOutlivesRequirement;
2019-12-13T00:19:17.9221141Z 
2019-12-13T00:19:17.9277894Z error[E0412]: cannot find type `ErrorReported` in this scope
2019-12-13T00:19:17.9278252Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:131:17
2019-12-13T00:19:17.9278518Z     |
2019-12-13T00:19:17.9278518Z     |
2019-12-13T00:19:17.9278844Z 131 |     ) -> Option<ErrorReported> {
2019-12-13T00:19:17.9279426Z     |
2019-12-13T00:19:17.9279746Z help: possible candidates are found in other modules, you can import them into scope
2019-12-13T00:19:17.9279968Z     |
2019-12-13T00:19:17.9279968Z     |
2019-12-13T00:19:17.9280254Z 3   | use rustc::util::common::ErrorReported;
2019-12-13T00:19:17.9280790Z 3   | use rustc_errors::ErrorReported;
2019-12-13T00:19:17.9281009Z     |
2019-12-13T00:19:17.9281063Z 
2019-12-13T00:19:17.9281063Z 
2019-12-13T00:19:17.9369562Z error[E0422]: cannot find struct, variant or union type `ClosureOutlivesRequirement` in this scope
2019-12-13T00:19:17.9370247Z     |
2019-12-13T00:19:17.9370247Z     |
2019-12-13T00:19:17.9370624Z 164 |                     propagated_outlives_requirements.push(ClosureOutlivesRequirement {
2019-12-13T00:19:17.9371360Z     |
2019-12-13T00:19:17.9371679Z help: possible candidate is found in another module, you can import it into scope
2019-12-13T00:19:17.9371964Z     |
2019-12-13T00:19:17.9371964Z     |
2019-12-13T00:19:17.9372288Z 3   | use rustc::mir::ClosureOutlivesRequirement;
2019-12-13T00:19:17.9372586Z 
2019-12-13T00:19:17.9372888Z error[E0425]: cannot find value `ErrorReported` in this scope
2019-12-13T00:19:17.9373206Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:196:14
2019-12-13T00:19:17.9373466Z     |
2019-12-13T00:19:17.9373466Z     |
2019-12-13T00:19:17.9373763Z 196 |         Some(ErrorReported)
2019-12-13T00:19:17.9374117Z     |              ^^^^^^^^^^^^^ not found in this scope
2019-12-13T00:19:17.9374618Z     |
2019-12-13T00:19:17.9375174Z help: possible candidates are found in other modules, you can import them into scope
2019-12-13T00:19:17.9375544Z     |
2019-12-13T00:19:17.9375836Z 3   | use rustc::util::common::ErrorReported;
2019-12-13T00:19:17.9376295Z 3   | use rustc_errors::ErrorReported;
2019-12-13T00:19:17.9376519Z     |
2019-12-13T00:19:17.9376550Z 
2019-12-13T00:19:18.2333190Z error: unused import: `Diagnostic`
2019-12-13T00:19:18.2333190Z error: unused import: `Diagnostic`
2019-12-13T00:19:18.2333559Z   --> src/librustc_mir/borrow_check/region_infer/mod.rs:21:20
2019-12-13T00:19:18.2333818Z    |
2019-12-13T00:19:18.2334121Z 21 | use rustc_errors::{Diagnostic, DiagnosticBuilder};
2019-12-13T00:19:18.2334616Z    |
2019-12-13T00:19:18.2335020Z    = note: `-D unused-imports` implied by `-D warnings`
2019-12-13T00:19:18.2335058Z 
2019-12-13T00:19:20.6131297Z error[E0308]: mismatched types
2019-12-13T00:19:20.6131297Z error[E0308]: mismatched types
2019-12-13T00:19:20.6131702Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:194:19
2019-12-13T00:19:20.6132037Z     |
2019-12-13T00:19:20.6132366Z 194 |         db.buffer(errors_buffer);
2019-12-13T00:19:20.6132860Z     |                   ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:20.6133167Z     |
2019-12-13T00:19:20.6133527Z     = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-13T00:19:20.6133873Z                found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:20.6179586Z error[E0624]: method `universal_upper_bound` is private
2019-12-13T00:19:20.6179952Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:216:36
2019-12-13T00:19:20.6180192Z     |
2019-12-13T00:19:20.6180192Z     |
2019-12-13T00:19:20.6180495Z 216 |             let upper_bound = self.universal_upper_bound(r);
2019-12-13T00:19:20.6180898Z 
2019-12-13T00:19:20.9785552Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-12-13T00:19:21.5334652Z error[E0308]: mismatched types
2019-12-13T00:19:21.5335182Z    --> src/librustc_mir/borrow_check/region_infer/mod.rs:914:29
2019-12-13T00:19:21.5335182Z    --> src/librustc_mir/borrow_check/region_infer/mod.rs:914:29
2019-12-13T00:19:21.5335459Z     |
2019-12-13T00:19:21.5335784Z 914 |                     .buffer(errors_buffer);
2019-12-13T00:19:21.5336288Z     |                             ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.5336841Z     |
2019-12-13T00:19:21.5337351Z     = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-13T00:19:21.5337736Z                found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.6262250Z error[E0308]: mismatched types
2019-12-13T00:19:21.6262690Z    --> src/librustc_mir/borrow_check/region_infer/mod.rs:931:29
2019-12-13T00:19:21.6262955Z     |
2019-12-13T00:19:21.6262955Z     |
2019-12-13T00:19:21.6263307Z 931 |                     .buffer(errors_buffer);
2019-12-13T00:19:21.6263788Z     |                             ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.6264092Z     |
2019-12-13T00:19:21.6264502Z     = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-13T00:19:21.6264840Z                found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.7619184Z error[E0308]: mismatched types
2019-12-13T00:19:21.7619629Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1366:63
2019-12-13T00:19:21.7619895Z      |
2019-12-13T00:19:21.7619895Z      |
2019-12-13T00:19:21.7620319Z 1366 |         outlives_suggestion.add_suggestion(body, self, infcx, errors_buffer, region_naming);
2019-12-13T00:19:21.7620907Z      |                                                               ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.7621197Z      |
2019-12-13T00:19:21.7621568Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-13T00:19:21.7621902Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.7707075Z error[E0624]: method `report_or_propagate_universal_region_error` is private
2019-12-13T00:19:21.7707640Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1440:18
2019-12-13T00:19:21.7708062Z      |
2019-12-13T00:19:21.7708448Z 1440 |             self.report_or_propagate_universal_region_error(
2019-12-13T00:19:21.7708448Z 1440 |             self.report_or_propagate_universal_region_error(
2019-12-13T00:19:21.7708844Z      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-13T00:19:21.7713781Z 
2019-12-13T00:19:21.8652606Z error[E0308]: mismatched types
2019-12-13T00:19:21.8653036Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1474:63
2019-12-13T00:19:21.8653409Z      |
2019-12-13T00:19:21.8653838Z 1474 |         outlives_suggestion.add_suggestion(body, self, infcx, errors_buffer, region_naming);
2019-12-13T00:19:21.8654408Z      |                                                               ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.8655014Z      |
2019-12-13T00:19:21.8655545Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-13T00:19:21.8655976Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.8719794Z error[E0624]: method `report_or_propagate_universal_region_error` is private
2019-12-13T00:19:21.8720202Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1580:14
2019-12-13T00:19:21.8720465Z      |
2019-12-13T00:19:21.8720805Z 1580 |         self.report_or_propagate_universal_region_error(
2019-12-13T00:19:21.8720805Z 1580 |         self.report_or_propagate_universal_region_error(
2019-12-13T00:19:21.8721192Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-13T00:19:21.8726244Z 
2019-12-13T00:19:21.9846039Z error[E0308]: mismatched types
2019-12-13T00:19:21.9846580Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1689:21
2019-12-13T00:19:21.9846900Z      |
2019-12-13T00:19:21.9847284Z 1689 |             .buffer(errors_buffer);
2019-12-13T00:19:21.9847795Z      |                     ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:21.9848090Z      |
2019-12-13T00:19:21.9848572Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-13T00:19:21.9848954Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-13T00:19:22.1447884Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-12-13T00:19:22.7293406Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-12-13T00:19:22.9672329Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-12-13T00:19:25.6225334Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
---
2019-12-13T00:19:30.1470075Z   local time: Fri Dec 13 00:19:30 UTC 2019
2019-12-13T00:19:30.2880365Z   network time: Fri, 13 Dec 2019 00:19:30 GMT
2019-12-13T00:19:30.2880644Z == end clock drift check ==
2019-12-13T00:19:31.6764596Z 
2019-12-13T00:19:31.6878253Z ##[error]Bash exited with code '1'.
2019-12-13T00:19:31.6908446Z ##[section]Starting: Checkout
2019-12-13T00:19:31.6909993Z ==============================================================================
2019-12-13T00:19:31.6910060Z Task         : Get sources
2019-12-13T00:19:31.6910102Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
