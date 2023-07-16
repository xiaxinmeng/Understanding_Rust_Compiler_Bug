plain
2020-02-17T02:03:01.1426923Z ========================== Starting Command Output ===========================
2020-02-17T02:03:01.1428378Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2a539baf-9c74-4980-9583-b4d25c8fe52a.sh
2020-02-17T02:03:01.1428412Z 
2020-02-17T02:03:01.1432566Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-17T02:03:01.1438336Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67460/merge to s
2020-02-17T02:03:01.1445662Z Task         : Get sources
2020-02-17T02:03:01.1445711Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T02:03:01.1445745Z Version      : 1.0.0
2020-02-17T02:03:01.1445778Z Author       : Microsoft
---
2020-02-17T02:03:02.0465742Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-17T02:03:02.0548343Z ##[command]git config gc.auto 0
2020-02-17T02:03:02.0633253Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-17T02:03:02.0685662Z ##[command]git config --get-all http.proxy
2020-02-17T02:03:02.0823870Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67460/merge:refs/remotes/pull/67460/merge
---
2020-02-17T02:09:57.8569402Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-17T02:09:59.4407890Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-17T02:10:23.0345614Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-02-17T02:10:23.4650225Z error[E0433]: failed to resolve: unresolved import
2020-02-17T02:10:23.4651500Z  --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:7:12
2020-02-17T02:10:23.4652719Z 7 | use crate::ty::error::ExpectedFound;
2020-02-17T02:10:23.4653313Z   |            ^^
2020-02-17T02:10:23.4653859Z   |            |
2020-02-17T02:10:23.4654444Z   |            unresolved import
2020-02-17T02:10:23.4654444Z   |            unresolved import
2020-02-17T02:10:23.4655052Z   |            help: a similar path exists: `rustc::ty`
2020-02-17T02:10:23.4655323Z 
2020-02-17T02:10:23.4682476Z error[E0433]: failed to resolve: unresolved import
2020-02-17T02:10:23.4683262Z  --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:8:12
2020-02-17T02:10:23.4684262Z 8 | use crate::ty::fold::TypeFoldable;
2020-02-17T02:10:23.4684939Z   |            ^^
2020-02-17T02:10:23.4685396Z   |            |
2020-02-17T02:10:23.4685852Z   |            unresolved import
2020-02-17T02:10:23.4685852Z   |            unresolved import
2020-02-17T02:10:23.4686365Z   |            help: a similar path exists: `rustc::ty`
2020-02-17T02:10:23.4686545Z 
2020-02-17T02:10:23.4756585Z error[E0433]: failed to resolve: unresolved import
2020-02-17T02:10:23.4757288Z   --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:10:12
2020-02-17T02:10:23.4758248Z 10 | use crate::util::common::ErrorReported;
2020-02-17T02:10:23.4758727Z    |            ^^^^
2020-02-17T02:10:23.4759171Z    |            |
2020-02-17T02:10:23.4759656Z    |            unresolved import
2020-02-17T02:10:23.4759656Z    |            unresolved import
2020-02-17T02:10:23.4760656Z    |            help: a similar path exists: `rustc::util`
2020-02-17T02:10:23.4760911Z 
2020-02-17T02:10:23.4883114Z error[E0432]: unresolved imports `crate::ty`, `crate::ty`
2020-02-17T02:10:23.4884079Z  --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:9:17
2020-02-17T02:10:23.4885004Z   |
2020-02-17T02:10:23.4885808Z 9 | use crate::ty::{self, Ty, TyCtxt};
2020-02-17T02:10:23.4886822Z   |            |
2020-02-17T02:10:23.4887277Z   |            unresolved import
2020-02-17T02:10:23.4887781Z   |            help: a similar path exists: `rustc::ty`
2020-02-17T02:10:23.4887956Z 
2020-02-17T02:10:23.4887956Z 
2020-02-17T02:10:23.5363816Z error[E0433]: failed to resolve: unresolved import
2020-02-17T02:10:23.5365208Z    --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:150:23
2020-02-17T02:10:23.5366218Z 150 |     type Map = crate::hir::map::Map<'tcx>;
2020-02-17T02:10:23.5366694Z     |                       ^^^
2020-02-17T02:10:23.5367150Z     |                       |
2020-02-17T02:10:23.5367612Z     |                       unresolved import
2020-02-17T02:10:23.5367612Z     |                       unresolved import
2020-02-17T02:10:23.5368141Z     |                       help: a similar path exists: `rustc::hir`
2020-02-17T02:10:23.5368316Z 
2020-02-17T02:10:23.6669614Z error[E0412]: cannot find type `ErrorReported` in this scope
2020-02-17T02:10:23.6670671Z   --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:20:76
2020-02-17T02:10:23.6671233Z    |
2020-02-17T02:10:23.6671780Z 20 |     pub(super) fn try_report_impl_not_conforming_to_trait(&self) -> Option<ErrorReported> {
2020-02-17T02:10:23.6672776Z    |
2020-02-17T02:10:23.6673309Z help: possible candidates are found in other modules, you can import them into scope
2020-02-17T02:10:23.6673716Z    |
2020-02-17T02:10:23.6674170Z 3  | use rustc::util::common::ErrorReported;
2020-02-17T02:10:23.6674170Z 3  | use rustc::util::common::ErrorReported;
2020-02-17T02:10:23.6674581Z    |
2020-02-17T02:10:23.6675022Z 3  | use rustc_errors::ErrorReported;
2020-02-17T02:10:23.6675438Z    |
2020-02-17T02:10:23.6675865Z help: you might be missing a type parameter
2020-02-17T02:10:23.6676242Z    |
2020-02-17T02:10:23.6676727Z 18 | impl<'a, 'tcx, ErrorReported> NiceRegionError<'a, 'tcx> {
2020-02-17T02:10:23.6677344Z 
2020-02-17T02:10:23.6694017Z error[E0425]: cannot find value `ErrorReported` in this scope
2020-02-17T02:10:23.6694017Z error[E0425]: cannot find value `ErrorReported` in this scope
2020-02-17T02:10:23.6694461Z   --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:47:45
2020-02-17T02:10:23.6695016Z 47 | ...                   return Some(ErrorReported);
2020-02-17T02:10:23.6695529Z    |                                   ^^^^^^^^^^^^^ not found in this scope
2020-02-17T02:10:23.6695756Z    |
2020-02-17T02:10:23.6696049Z help: possible candidates are found in other modules, you can import them into scope
2020-02-17T02:10:23.6696049Z help: possible candidates are found in other modules, you can import them into scope
2020-02-17T02:10:23.6696288Z    |
2020-02-17T02:10:23.6696581Z 3  | use rustc::util::common::ErrorReported;
2020-02-17T02:10:23.6696799Z    |
2020-02-17T02:10:23.6697092Z 3  | use rustc_errors::ErrorReported;
2020-02-17T02:10:23.6697306Z    |
2020-02-17T02:10:23.6697339Z 
2020-02-17T02:10:23.6716607Z error[E0422]: cannot find struct, variant or union type `ExpectedFound` in this scope
2020-02-17T02:10:23.6717078Z    --> src/librustc_infer/infer/error_reporting/nice_region_error/trait_impl_difference.rs:127:57
2020-02-17T02:10:23.6717358Z     |
2020-02-17T02:10:23.6717712Z 127 |             .enter(|infcx| infcx.expected_found_str_ty(&ExpectedFound { expected, found }))
2020-02-17T02:10:23.6718381Z     |
2020-02-17T02:10:23.6718685Z help: possible candidate is found in another module, you can import it into scope
2020-02-17T02:10:23.6718942Z     |
2020-02-17T02:10:23.6719243Z 3   | use rustc::ty::error::ExpectedFound;
---
2020-02-17T02:10:23.9881717Z For more information about an error, try `rustc --explain E0412`.
2020-02-17T02:10:23.9929744Z error: could not compile `rustc_infer`.
2020-02-17T02:10:23.9930116Z warning: build failed, waiting for other jobs to finish...
2020-02-17T02:10:25.6035652Z error: build failed
2020-02-17T02:10:25.6061456Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-17T02:10:25.6070250Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-17T02:10:25.6070416Z Build completed unsuccessfully in 0:05:03
2020-02-17T02:10:25.6124241Z == clock drift check ==
2020-02-17T02:10:25.6138494Z   local time: Mon Feb 17 02:10:25 UTC 2020
2020-02-17T02:10:25.6138494Z   local time: Mon Feb 17 02:10:25 UTC 2020
2020-02-17T02:10:25.8965330Z   network time: Mon, 17 Feb 2020 02:10:25 GMT
2020-02-17T02:10:25.8965693Z == end clock drift check ==
2020-02-17T02:10:26.4621015Z 
2020-02-17T02:10:26.4722424Z ##[error]Bash exited with code '1'.
2020-02-17T02:10:26.4737542Z ##[section]Finishing: Run build
2020-02-17T02:10:26.4752555Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67460/merge to s
2020-02-17T02:10:26.4754585Z Task         : Get sources
2020-02-17T02:10:26.4754634Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T02:10:26.4754681Z Version      : 1.0.0
2020-02-17T02:10:26.4754743Z Author       : Microsoft
2020-02-17T02:10:26.4754743Z Author       : Microsoft
2020-02-17T02:10:26.4754792Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-17T02:10:26.4754844Z ==============================================================================
2020-02-17T02:10:26.9018906Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-17T02:10:26.9041596Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67460/merge to s
2020-02-17T02:10:26.9173668Z Cleaning up task key
2020-02-17T02:10:26.9174567Z Start cleaning up orphan processes.
2020-02-17T02:10:26.9299574Z Terminate orphan process: pid (3525) (python)
2020-02-17T02:10:26.9497591Z ##[section]Finishing: Finalize Job
