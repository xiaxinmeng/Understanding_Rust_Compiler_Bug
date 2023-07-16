plain
2019-12-16T16:54:32.8309764Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T16:54:32.8576186Z ##[command]git config gc.auto 0
2019-12-16T16:54:32.8672100Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T16:54:32.8739735Z ##[command]git config --get-all http.proxy
2019-12-16T16:54:32.8907127Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-16T17:03:08.3312255Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-12-16T17:03:11.1136586Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-12-16T17:03:11.6615207Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-16T17:03:16.0426316Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-12-16T17:03:17.1842977Z error: method is never used: `get_var_name_and_span_for_region`
2019-12-16T17:03:17.1843396Z   --> src/librustc_mir/borrow_check/diagnostics/var_name.rs:10:5
2019-12-16T17:03:17.1844660Z 10 | /     crate fn get_var_name_and_span_for_region(
2019-12-16T17:03:17.1845039Z 11 | |         &self,
2019-12-16T17:03:17.1867223Z 12 | |         tcx: TyCtxt<'tcx>,
2019-12-16T17:03:17.1867684Z 13 | |         body: &Body<'tcx>,
2019-12-16T17:03:17.1867684Z 13 | |         body: &Body<'tcx>,
2019-12-16T17:03:17.1867995Z ...  |
2019-12-16T17:03:17.1868311Z 33 | |             })
2019-12-16T17:03:17.1868606Z 34 | |     }
2019-12-16T17:03:17.1868887Z    | |_____^
2019-12-16T17:03:17.1869112Z    |
2019-12-16T17:03:17.1869397Z    = note: `-D dead-code` implied by `-D warnings`
2019-12-16T17:03:17.1869452Z 
2019-12-16T17:03:17.1869710Z error: method is never used: `was_named`
2019-12-16T17:03:17.1870319Z     |
2019-12-16T17:03:17.1870319Z     |
2019-12-16T17:03:17.1870621Z 109 |     crate fn was_named(&self) -> bool {
2019-12-16T17:03:17.1871025Z 
2019-12-16T17:03:17.1871025Z 
2019-12-16T17:03:17.1954008Z error: enum is never used: `SuggestedConstraint`
2019-12-16T17:03:17.1954421Z   --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:23:1
2019-12-16T17:03:17.1955038Z 23 | enum SuggestedConstraint {
2019-12-16T17:03:17.1955312Z    | ^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T17:03:17.1955406Z 
2019-12-16T17:03:17.1955406Z 
2019-12-16T17:03:17.1959177Z error: struct is never constructed: `OutlivesSuggestionBuilder`
2019-12-16T17:03:17.1959496Z   --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:38:1
2019-12-16T17:03:17.1959759Z    |
2019-12-16T17:03:17.1960059Z 38 | pub struct OutlivesSuggestionBuilder<'a> {
2019-12-16T17:03:17.1960385Z 
2019-12-16T17:03:17.1972880Z error: method is never used: `new`
2019-12-16T17:03:17.1973215Z   --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:54:5
2019-12-16T17:03:17.1973471Z    |
2019-12-16T17:03:17.1973471Z    |
2019-12-16T17:03:17.1975134Z 54 | /     crate fn new(
2019-12-16T17:03:17.1975703Z 55 | |         mir_def_id: DefId,
2019-12-16T17:03:17.1976202Z 56 | |         local_names: &'a IndexVec<Local, Option<Symbol>>,
2019-12-16T17:03:17.1976822Z ...  |
2019-12-16T17:03:17.1977134Z 62 | |         }
2019-12-16T17:03:17.1977428Z 63 | |     }
2019-12-16T17:03:17.1977698Z    | |_____^
2019-12-16T17:03:17.1977698Z    | |_____^
2019-12-16T17:03:17.1977747Z 
2019-12-16T17:03:17.1978023Z error: method is never used: `region_name_is_suggestable`
2019-12-16T17:03:17.1978327Z   --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:75:5
2019-12-16T17:03:17.1978731Z    |
2019-12-16T17:03:17.1979089Z 75 |     fn region_name_is_suggestable(name: &RegionName) -> bool {
2019-12-16T17:03:17.1979512Z 
2019-12-16T17:03:17.1979789Z error: method is never used: `region_vid_to_name`
2019-12-16T17:03:17.1980093Z    --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:98:5
2019-12-16T17:03:17.1980380Z     |
2019-12-16T17:03:17.1980380Z     |
2019-12-16T17:03:17.1980754Z 98  | /     fn region_vid_to_name(
2019-12-16T17:03:17.1981673Z 99  | |         &self,
2019-12-16T17:03:17.1982112Z 100 | |         errctx: &ErrorReportingCtx<'_, '_, '_>,
2019-12-16T17:03:17.1982451Z 101 | |         renctx: &mut RegionErrorNamingCtx,
2019-12-16T17:03:17.1983057Z 107 | |             .filter(Self::region_name_is_suggestable)
2019-12-16T17:03:17.1983354Z 108 | |     }
2019-12-16T17:03:17.1983641Z     | |_____^
2019-12-16T17:03:17.1983678Z 
---
2019-12-16T17:03:17.1987338Z 
2019-12-16T17:03:17.1987596Z error: method is never used: `collect_constraint`
2019-12-16T17:03:17.1987915Z    --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:203:5
2019-12-16T17:03:17.1988143Z     |
2019-12-16T17:03:17.1988473Z 203 |     crate fn collect_constraint(&mut self, fr: RegionVid, outlived_fr: RegionVid) {
2019-12-16T17:03:17.1988878Z 
2019-12-16T17:03:17.1989153Z error: method is never used: `intermediate_suggestion`
2019-12-16T17:03:17.1989475Z    --> src/librustc_mir/borrow_check/diagnostics/outlives_suggestion.rs:212:5
2019-12-16T17:03:17.1989703Z     |
2019-12-16T17:03:17.1989703Z     |
2019-12-16T17:03:17.1990025Z 212 | /     crate fn intermediate_suggestion(
2019-12-16T17:03:17.1990347Z 213 | |         &mut self,
2019-12-16T17:03:17.1990684Z 214 | |         errctx: &ErrorReportingCtx<'_, '_, '_>,
2019-12-16T17:03:17.1991264Z 215 | |         errci: &ErrorConstraintInfo,
2019-12-16T17:03:17.1991917Z 232 | |         }
2019-12-16T17:03:17.1992212Z 233 | |     }
2019-12-16T17:03:17.1992483Z     | |_____^
2019-12-16T17:03:17.1992518Z 
---
2019-12-16T17:03:17.2150376Z 238 | |         &self,
2019-12-16T17:03:17.2150728Z 239 | |         body: &Body<'tcx>,
2019-12-16T17:03:17.2151354Z 240 | |         region_infcx: &RegionInferenceContext<'tcx>,
2019-12-16T17:03:17.2151701Z ...   |
2019-12-16T17:03:17.2152037Z 317 | |         diag.buffer(errors_buffer);
2019-12-16T17:03:17.2152613Z     | |_____^
2019-12-16T17:03:17.2152862Z 
2019-12-16T17:03:17.2183759Z error: field is never used: `region_infcx`
2019-12-16T17:03:17.2184137Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:115:5
2019-12-16T17:03:17.2184137Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:115:5
2019-12-16T17:03:17.2184547Z     |
2019-12-16T17:03:17.2184936Z 115 |     pub(super) region_infcx: &'b RegionInferenceContext<'tcx>,
2019-12-16T17:03:17.2185319Z 
2019-12-16T17:03:17.2185582Z error: method is never used: `report_error`
2019-12-16T17:03:17.2185900Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:460:5
2019-12-16T17:03:17.2186135Z     |
2019-12-16T17:03:17.2186135Z     |
2019-12-16T17:03:17.2186486Z 460 | /     pub(in crate::borrow_check) fn report_error<'a>(
2019-12-16T17:03:17.2186847Z 461 | |         &'a self,
2019-12-16T17:03:17.2187168Z 462 | |         body: &Body<'tcx>,
2019-12-16T17:03:17.2187513Z 463 | |         local_names: &IndexVec<Local, Option<Symbol>>,
2019-12-16T17:03:17.2188081Z 533 | |         }
2019-12-16T17:03:17.2188376Z 534 | |     }
2019-12-16T17:03:17.2188660Z     | |_____^
2019-12-16T17:03:17.2188696Z 
2019-12-16T17:03:17.2188696Z 
2019-12-16T17:03:17.2188959Z error: method is never used: `report_fnmut_error`
2019-12-16T17:03:17.2189514Z     |
2019-12-16T17:03:17.2189822Z 579 | /     fn report_fnmut_error(
2019-12-16T17:03:17.2190139Z 580 | |         &self,
2019-12-16T17:03:17.2190139Z 580 | |         &self,
2019-12-16T17:03:17.2190480Z 581 | |         errctx: &ErrorReportingCtx<'_, '_, 'tcx>,
2019-12-16T17:03:17.2190808Z 582 | |         errci: &ErrorConstraintInfo,
2019-12-16T17:03:17.2191719Z 627 | |         diag
2019-12-16T17:03:17.2192028Z 628 | |     }
2019-12-16T17:03:17.2192311Z     | |_____^
2019-12-16T17:03:17.2192347Z 
2019-12-16T17:03:17.2192347Z 
2019-12-16T17:03:17.2219863Z error: method is never used: `report_escaping_data_error`
2019-12-16T17:03:17.2220247Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:642:5
2019-12-16T17:03:17.2220487Z     |
2019-12-16T17:03:17.2220810Z 642 | /     fn report_escaping_data_error(
2019-12-16T17:03:17.2221733Z 643 | |         &self,
2019-12-16T17:03:17.2222083Z 644 | |         errctx: &ErrorReportingCtx<'_, '_, 'tcx>,
2019-12-16T17:03:17.2222543Z 645 | |         errci: &ErrorConstraintInfo,
2019-12-16T17:03:17.2223189Z 721 | |         diag
2019-12-16T17:03:17.2223501Z 722 | |     }
2019-12-16T17:03:17.2223759Z     | |_____^
2019-12-16T17:03:17.2227455Z 
2019-12-16T17:03:17.2227455Z 
2019-12-16T17:03:17.2307132Z error: method is never used: `report_general_error`
2019-12-16T17:03:17.2307581Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:739:5
2019-12-16T17:03:17.2307823Z     |
2019-12-16T17:03:17.2308173Z 739 | /     fn report_general_error(
2019-12-16T17:03:17.2308493Z 740 | |         &self,
2019-12-16T17:03:17.2308835Z 741 | |         errctx: &ErrorReportingCtx<'_, '_, 'tcx>,
2019-12-16T17:03:17.2309181Z 742 | |         errci: &ErrorConstraintInfo,
2019-12-16T17:03:17.2309740Z 794 | |         diag
2019-12-16T17:03:17.2310071Z 795 | |     }
2019-12-16T17:03:17.2310329Z     | |_____^
2019-12-16T17:03:17.2314369Z 
2019-12-16T17:03:17.2314369Z 
2019-12-16T17:03:17.2319575Z error: method is never used: `add_static_impl_trait_suggestion`
2019-12-16T17:03:17.2319921Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:806:5
2019-12-16T17:03:17.2320160Z     |
2019-12-16T17:03:17.2320511Z 806 | /     fn add_static_impl_trait_suggestion(
2019-12-16T17:03:17.2320820Z 807 | |         &self,
2019-12-16T17:03:17.2321147Z 808 | |         infcx: &InferCtxt<'_, 'tcx>,
2019-12-16T17:03:17.2321520Z 809 | |         diag: &mut DiagnosticBuilder<'_>,
2019-12-16T17:03:17.2322466Z 886 | |         }
2019-12-16T17:03:17.2322768Z 887 | |     }
2019-12-16T17:03:17.2323024Z     | |_____^
2019-12-16T17:03:17.2385140Z 
2019-12-16T17:03:17.2385140Z 
2019-12-16T17:03:17.2386165Z error: method is never used: `is_closure_fn_mut`
2019-12-16T17:03:17.2386504Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:992:5
2019-12-16T17:03:17.2387061Z     |
2019-12-16T17:03:17.2387435Z 992 |     crate fn is_closure_fn_mut(&self, infcx: &InferCtxt<'_, 'tcx>, fr: RegionVid) -> bool {
2019-12-16T17:03:17.2387951Z 
2019-12-16T17:03:17.2826780Z error: usage of qualified `ty::Ty<'tcx>`
2019-12-16T17:03:17.2827184Z   --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:96:20
2019-12-16T17:03:17.2827433Z    |
2019-12-16T17:03:17.2827433Z    |
2019-12-16T17:03:17.2827750Z 96 |         hidden_ty: ty::Ty<'tcx>,
2019-12-16T17:03:17.2828110Z    |                    ^^^^^^^^^^^^ help: try using it unqualified: `Ty<'tcx>`
2019-12-16T17:03:17.2828396Z    |
2019-12-16T17:03:17.2828706Z    = note: `-D rustc::usage-of-qualified-ty` implied by `-D warnings`
2019-12-16T17:03:18.5244214Z error: aborting due to 19 previous errors
2019-12-16T17:03:18.5244899Z 
2019-12-16T17:03:18.5253879Z error: could not compile `rustc_mir`.
2019-12-16T17:03:18.5254644Z warning: build failed, waiting for other jobs to finish...
2019-12-16T17:03:18.5254644Z warning: build failed, waiting for other jobs to finish...
2019-12-16T17:03:21.7031392Z error: build failed
2019-12-16T17:03:21.7074119Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-16T17:03:21.7080798Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-16T17:03:21.7081182Z Build completed unsuccessfully in 0:05:32
2019-12-16T17:03:21.7136710Z == clock drift check ==
2019-12-16T17:03:21.7169453Z   local time: Mon Dec 16 17:03:21 UTC 2019
2019-12-16T17:03:21.7169453Z   local time: Mon Dec 16 17:03:21 UTC 2019
2019-12-16T17:03:21.9969044Z   network time: Mon, 16 Dec 2019 17:03:21 GMT
2019-12-16T17:03:21.9970053Z == end clock drift check ==
2019-12-16T17:03:23.3906945Z 
2019-12-16T17:03:23.4031765Z ##[error]Bash exited with code '1'.
2019-12-16T17:03:23.4069043Z ##[section]Starting: Checkout
2019-12-16T17:03:23.4070990Z ==============================================================================
2019-12-16T17:03:23.4071051Z Task         : Get sources
2019-12-16T17:03:23.4071138Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
