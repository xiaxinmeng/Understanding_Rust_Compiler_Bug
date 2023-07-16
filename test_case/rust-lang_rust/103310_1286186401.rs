plain

   Doc-tests rustc_hir_typeck

running 37 tests
test src/closure.rs - closure::FnCtxt<'a,'tcx>::sig_of_closure_with_expectation (line 379) ... ignored
test src/coercion.rs - coercion (line 24) ... ignored
test src/coercion.rs - coercion::CoerceMany (line 1335) ... ignored
test src/demand.rs - demand::FnCtxt<'a,'tcx>::can_use_as_ref (line 579) ... ignored
test src/fallback.rs - fallback::FnCtxt<'_,'tcx>::calculate_diverging_fallback (line 171) ... ignored
test src/inherited.rs - inherited::Inherited (line 24) ... ignored
test src/lib.rs - TupleArgumentsFlag (line 452) ... ignored
test src/mem_categorization.rs - mem_categorization (line 12) ... ignored
test src/method/probe.rs - method::probe::Pick::autoderefs (line 203) ... ignored
test src/method/probe.rs - method::probe::ProbeContext<'a,'tcx>::collapse_candidates_to_trait_pick (line 1636) ... ignored
test src/pat.rs - pat::FnCtxt<'a,'tcx>::check_pat_slice (line 1979) ... ignored
test src/upvar.rs - upvar (line 8) ... ignored
test src/fn_ctxt/suggestions.rs - fn_ctxt::suggestions::FnCtxt<'a,'tcx>::consider_removing_semicolon (line 1173) - compile fail ... ok
test src/upvar.rs - upvar::FnCtxt<'a,'tcx>::compute_min_captures (line 465) ... ignored
test src/upvar.rs - upvar::FnCtxt<'a,'tcx>::compute_min_captures (line 491) ... ignored
test src/fn_ctxt/suggestions.rs - fn_ctxt::suggestions::FnCtxt<'a,'tcx>::suggest_fn_call (line 70) - compile fail ... ok
test src/demand.rs - demand::FnCtxt<'a,'tcx>::check_ref (line 699) - compile fail ... ok
test src/upvar.rs - upvar::FnCtxt<'a,'tcx>::has_significant_drop_outside_of_captures (line 1284) ... ignored
test src/upvar.rs - upvar::FnCtxt<'a,'tcx>::has_significant_drop_outside_of_captures (line 1322) ... ignored
test src/upvar.rs - upvar::InferBorrowKind::capture_information (line 1743) ... ignored
test src/demand.rs - demand::FnCtxt<'a,'tcx>::can_use_as_ref (line 569) - compile fail ... ok
test src/generator_interior/drop_ranges/cfg_build.rs - generator_interior::drop_ranges::cfg_build::DropRangeVisitor (line 77) - compile fail ... ok
test src/upvar.rs - upvar::determine_capture_info (line 2078) - compile ... ok
test src/fn_ctxt/suggestions.rs - fn_ctxt::suggestions::FnCtxt<'a,'tcx>::try_suggest_return_impl_trait (line 730) - compile fail ... ok
test src/upvar.rs - upvar::InferBorrowKind::capture_information (line 1727) - compile ... ok
test src/fn_ctxt/suggestions.rs - fn_ctxt::suggestions::FnCtxt<'a,'tcx>::suggest_missing_semicolon (line 597) - compile fail ... ok
test src/fn_ctxt/suggestions.rs - fn_ctxt::suggestions::FnCtxt<'a,'tcx>::suggest_missing_return_type (line 658) - compile fail ... ok
test src/fallback.rs - fallback::FnCtxt<'_,'tcx>::calculate_diverging_fallback (line 145) ... ok
test src/expr_use_visitor.rs - expr_use_visitor::ExprUseVisitor<'a,'tcx>::walk_captures (line 737) ... ok
test src/mem_categorization.rs - mem_categorization (line 43) ... ok
test src/lib.rs - TupleArgumentsFlag (line 457) ... ok
test src/lib.rs - TupleArgumentsFlag (line 448) ... ok
test src/generator_interior/drop_ranges/cfg_build.rs - generator_interior::drop_ranges::cfg_build::DropRangeVisitor (line 69) ... ok
test src/generator_interior/drop_ranges/cfg_build.rs - generator_interior::drop_ranges::cfg_build::DropRangeVisitor (line 62) ... ok
test src/upvar.rs - upvar::FnCtxt<'a,'tcx>::has_significant_drop_outside_of_captures (line 1253) ... ok
test src/upvar.rs - upvar::FnCtxt<'a,'tcx>::compute_min_captures (line 446) ... ok
test src/upvar.rs - upvar::truncate_capture_for_optimization (line 2226) ... ok
test result: ok. 20 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.20s

   Doc-tests rustc_incremental

---
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
 Documenting rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
 Documenting rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
 Documenting rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unresolved link to `crate::check::FnCtxt`
    |
    |
109 | /// [`FnCtxt`]: crate::check::FnCtxt
    |                 ^^^^^^^^^^^^^^^^^^^^ no item named `FnCtxt` in module `check`
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: could not document `rustc_hir_analysis`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_hir_analysis compiler/rustc_hir_analysis/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=f7cf174792143806 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-a944b458c0ed8963.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-ad76b7b49185d3fe.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-aeaae4366cda3ff0.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f111def0f66356c2.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-68eb81595ada3345.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-59b5e85e2c18045d.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-0154684f1f480fec.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-707ab942df339192.rmeta --extern rustc_hir_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_pretty-6e7f252ba889a882.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-5915b49fa8f2008c.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-e06e974734c975b7.rmeta --extern rustc_lint=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-0b7a7866a38cc4c9.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-f297a551c4be84fd.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-5589c5c13da3b661.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-d3e125dfcd1acfa6.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-1a4dc4a17432c7e0.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-866f9a0b124b375e.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-ca4c77d044064363.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-3afe7698d6c094f5.rmeta --extern rustc_type_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_type_ir-ab40d743cf926e09.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-514c0d16dcc4ec4a.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-f4be7b713f095226.rmeta --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0-nightly
  (434cd54dd
  2022-10-20)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 1)
[RUSTC-TIMING] rustc_hir_analysis test:false 6.359
[RUSTC-TIMING] rustc_codegen_ssa test:false 4.475
[RUSTC-TIMING] rustc_mir_transform test:false 4.717
[RUSTC-TIMING] rustc_borrowck test:false 6.976
