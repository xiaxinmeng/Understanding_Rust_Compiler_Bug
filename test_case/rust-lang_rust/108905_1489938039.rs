plain
test [ui] tests/ui/coercion/coerce-block-tail-26978.rs ... ok
test [ui] tests/ui/codemap_tests/unicode_3.rs ... ok
test [ui] tests/ui/coercion/coerce-block-tail-57749.rs ... ok
test [ui] tests/ui/coercion/coerce-block-tail-83783.rs ... ok
test [ui] tests/ui/coercion/coerce-issue-49593-box-never-windows.rs#fallback ... ignored, only executed when the operative system is windows (the number of `Error` impls is platform-dependent)
test [ui] tests/ui/coercion/coerce-issue-49593-box-never-windows.rs#nofallback ... ignored, only executed when the operative system is windows (the number of `Error` impls is platform-dependent)
test [ui] tests/ui/coercion/coerce-expect-unsized-ascribed.rs ... ok
test [ui] tests/ui/coercion/coerce-block-tail-83850.rs ... ok
test [ui] tests/ui/coercion/coerce-issue-49593-box-never.rs#fallback ... ok
test [ui] tests/ui/coercion/coerce-issue-49593-box-never.rs#nofallback ... ok
---
test [ui] tests/ui/consts/offset_from.rs ... ok
test [ui] tests/ui/consts/static_mut_containing_mut_ref2.rs#mut_refs ... ok
test [ui] tests/ui/consts/static_mut_containing_mut_ref.rs ... ok
test [ui] tests/ui/consts/static_mut_containing_mut_ref2.rs#stock ... ok
test [ui] tests/ui/consts/std/alloc.rs ... ignored, ignored when building with debug assertions ((the debug assertions change the error))
test [ui] tests/ui/consts/packed_pattern.rs ... ok
test [ui] tests/ui/consts/std/cell.rs ... ok
test [ui] tests/ui/consts/packed_pattern2.rs ... ok
test [ui] tests/ui/consts/too_generic_eval_ice.rs ... ok
---
test [ui] tests/ui/invalid/invalid-inline.rs ... ok
test [ui] tests/ui/intrinsics/intrinsic-atomics-cc.rs ... ok
test [ui] tests/ui/invalid/invalid-macro-matcher.rs ... ok
test [ui] tests/ui/invalid/invalid-no-sanitize.rs ... ok
test [ui] tests/ui/io-checks/inaccessbile-temp-dir.rs ... ignored, ignored when the architecture is arm (the file-system issues do not replicate here, at least on armhf-gnu)
test [ui] tests/ui/io-checks/non-ice-error-on-worker-io-fail.rs ... ignored, ignored when the architecture is arm (the file-system issues do not replicate here, at least on armhf-gnu)
test [ui] tests/ui/invalid/invalid-plugin-attr.rs ... ok
test [ui] tests/ui/invalid/invalid-path-in-const.rs ... ok
test [ui] tests/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs ... ok
test [ui] tests/ui/invalid/invalid_rustc_layout_scalar_valid_range.rs ... ok
---
test [ui] tests/ui/issues/issue-6936.rs ... ok
test [ui] tests/ui/issues/issue-69396-const-no-type-in-macro.rs ... ok
test [ui] tests/ui/issues/issue-69455.rs ... ok
test [ui] tests/ui/issues/issue-69602-type-err-during-codegen-ice.rs ... ok
test [ui] tests/ui/issues/issue-70093/issue-70093-link-directives.rs ... ignored, ignored when cross-compiling (default-linker-libraries=yes doesn't play well with cross compiling)
test [ui] tests/ui/issues/issue-70093/issue-70093.rs ... ignored, ignored when cross-compiling (default-linker-libraries=yes doesn't play well with cross compiling)
test [ui] tests/ui/issues/issue-70381.rs ... ok
test [ui] tests/ui/issues/issue-61696.rs ... ok
test [ui] tests/ui/issues/issue-7044.rs ... ok
test [ui] tests/ui/issues/issue-7061.rs ... ok
---
test [ui] tests/ui/iterators/iter-range.rs ... ok
test [ui] tests/ui/iterators/string.rs ... ok
test [ui] tests/ui/json/json-and-color.rs ... ok
test [ui] tests/ui/iterators/vec-on-unimplemented.rs ... ok
test [ui] tests/ui/json/json-bom-plus-crlf-multifile-aux.rs ... ignored, ignored always (Not a test. Used by other tests)
test [ui] tests/ui/json/json-bom-plus-crlf-multifile.rs ... ok
test [ui] tests/ui/json/json-bom-plus-crlf.rs ... ok
test [ui] tests/ui/json/json-invalid.rs ... ok
test [ui] tests/ui/json/json-multiple.rs ... ok
---
test [ui] tests/ui/lint/command-line-lint-group-deny.rs ... ok
test [ui] tests/ui/lint/command-line-register-lint-tool.rs ... ok
test [ui] tests/ui/lint/command-line-register-unknown-lint-tool.rs ... ok
test [ui] tests/ui/lint/command-line-lint-group-warn.rs ... ok
test [ui] tests/ui/lint/dead-code/closure-bang.rs ... ignored, ignored always (FIXME(#20574))
test [ui] tests/ui/lint/crate_level_only_lint.rs ... ok
test [ui] tests/ui/lint/dead-code/anon-const-in-pat.rs ... ok
test [ui] tests/ui/lint/clashing-extern-fn.rs ... ok
test [ui] tests/ui/lint/dead-code/basic.rs ... ok
---
test [ui] tests/ui/lint/dead-code/lint-dead-code-2.rs ... ok
test [ui] tests/ui/limits/huge-struct.rs ... ok
test [ui] tests/ui/lint/dead-code/lint-dead-code-3.rs ... ok
test [ui] tests/ui/lint/dead-code/lint-dead-code-4.rs ... ok
test [ui] tests/ui/lint/dead-code/self-assign.rs ... ignored, ignored always (FIXME(81658, 83171))
test [ui] tests/ui/lint/dead-code/lint-dead-code-6.rs ... ok
test [ui] tests/ui/lint/dead-code/multiple-dead-codes-in-the-same-struct.rs ... ok
test [ui] tests/ui/lint/dead-code/newline-span.rs ... ok
test [ui] tests/ui/lint/dead-code/trait-impl.rs ... ok
test [ui] tests/ui/lint/dead-code/trait-impl.rs ... ok
test [ui] tests/ui/lint/dead-code/tuple-struct-field.rs ... ok
test [ui] tests/ui/lint/dead-code/type-alias.rs ... ok
test [ui] tests/ui/lint/dead-code/type-in-foreign.rs ... ok
test [ui] tests/ui/lint/dead-code/unused-enum.rs ... ok
test [ui] tests/ui/lint/dead-code/unused-struct-variant.rs ... ok
test [ui] tests/ui/lint/dead-code/with-core-crate.rs ... ok
test [ui] tests/ui/lint/dead-code/unused-variant.rs ... ok
test [ui] tests/ui/lint/expansion-time-include.rs ... ignored, ignored always (auxiliary file for expansion-time.rs)
test [ui] tests/ui/lint/empty-lint-attributes.rs ... ok
test [ui] tests/ui/lint/expansion-time.rs ... ok
test [ui] tests/ui/lint/expr_attr_paren_order.rs ... ok
test [ui] tests/ui/lint/dead-code/unused-variant-pub.rs ... ok
---
test [ui] tests/ui/lto/all-crates.rs ... ok
test [ui] tests/ui/macros/format-parse-errors.rs ... ok
test [ui] tests/ui/macros/global-asm.rs ... ok
test [ui] tests/ui/macros/format-unused-lables.rs ... ok
test [ui] tests/ui/macros/include-single-expr-helper-1.rs ... ignored, ignored always (auxiliary file for include-single-expr.rs)
test [ui] tests/ui/macros/include-single-expr-helper.rs ... ignored, ignored always (auxiliary file for include-single-expr.rs)
test [ui] tests/ui/lto/fat-lto.rs ... ok
test [ui] tests/ui/macros/issue-102878.rs ... ok
test [ui] tests/ui/macros/issue-103529.rs ... ok
test [ui] tests/ui/macros/issue-104769-concat_bytes-invalid-literal.rs ... ok
---
test [ui] tests/ui/macros/issue-6596-1.rs ... ok
test [ui] tests/ui/macros/issue-63102.rs ... ok
test [ui] tests/ui/macros/issue-68058.rs ... ok
test [ui] tests/ui/macros/issue-25274.rs ... ok
test [ui] tests/ui/macros/issue-69838-dir/bar.rs ... ignored, ignored always (this is an auxiliary file as part of another test.)
test [ui] tests/ui/macros/issue-69838-dir/included.rs ... ignored, ignored always (this is an auxiliary file as part of another test.)
test [ui] tests/ui/macros/issue-33185.rs ... ok
test [ui] tests/ui/macros/issue-70446.rs ... ok
test [ui] tests/ui/macros/issue-69838-mods-relative-to-included-path.rs ... ok
test [ui] tests/ui/macros/issue-77475.rs ... ok
---
test [ui] tests/ui/missing/missing-derivable-attr.rs ... ok
test [ui] tests/ui/missing/missing-fields-in-struct-pattern.rs ... ok
test [ui] tests/ui/mismatched_types/suggest-removing-tuple-struct-field.rs ... ok
test [ui] tests/ui/mir/mir_temp_promotions.rs ... ok
test [ui] tests/ui/missing_non_modrs_mod/foo.rs ... ignored, ignored always (this is just a helper for the real test in this dir)
test [ui] tests/ui/missing_non_modrs_mod/foo_inline.rs ... ignored, ignored always (this is just a helper for the real test in this dir)
test [ui] tests/ui/missing-trait-bounds/issue-35677.rs ... ok
test [ui] tests/ui/missing-trait-bounds/missing-trait-bound-for-op.rs ... ok
test [ui] tests/ui/missing-trait-bounds/missing-trait-bounds-for-method-call.rs ... ok
test [ui] tests/ui/missing/missing-comma-in-match.rs ... ok
---
test [ui] tests/ui/module-macro_use-arguments.rs ... ok
test [ui] tests/ui/modules/issue-56411.rs ... ok
test [ui] tests/ui/mir/validate/needs-reveal-all.rs ... ok
test [ui] tests/ui/missing/missing-items/m2.rs ... ok
test [ui] tests/ui/modules/mod_file_aux.rs ... ignored, ignored always (Not a test. Used by other tests)
test [ui] tests/ui/missing/missing-macro-use.rs ... ok
test [ui] tests/ui/missing_debug_impls.rs ... ok
test [ui] tests/ui/modules/path-macro.rs ... ok
test [ui] tests/ui/modules/path-no-file-name.rs ... ok
test [ui] tests/ui/modules/path-no-file-name.rs ... ok
test [ui] tests/ui/modules/path-invalid-form.rs ... ok
test [ui] tests/ui/modules_and_files_visibility/mod_file_aux.rs ... ignored, ignored always (Not a test. Used by other tests)
test [ui] tests/ui/modules_and_files_visibility/mod_file_correct_spans.rs ... ok
test [ui] tests/ui/modules_and_files_visibility/mod_file_correct_spans.rs ... ok
test [ui] tests/ui/modules_and_files_visibility/mod_file_disambig_aux.rs ... ignored, ignored always (not a test. aux file)
test [ui] tests/ui/modules_and_files_visibility/mod_file_disambig_aux/mod.rs ... ignored, ignored always (not a test. aux file)
test [ui] tests/ui/missing-trait-bounds/issue-69725.rs ... ok
test [ui] tests/ui/moves/borrow-closures-instead-of-move.rs ... ok
test [ui] tests/ui/moves/issue-46099-move-in-macro.rs ... ok
test [ui] tests/ui/moves/issue-72649-uninit-in-loop.rs ... ok
---
test [ui] tests/ui/no-warn-on-field-replace-issue-34101.rs ... ok
test [ui] tests/ui/no_send-enum.rs ... ok
test [ui] tests/ui/no_send-rc.rs ... ok
test [ui] tests/ui/no_share-enum.rs ... ok
test [ui] tests/ui/non_modrs_mods/foors_mod.rs ... ignored, ignored always (not a test, used by non_modrs_mods.rs)
test [ui] tests/ui/non_modrs_mods_and_inline_mods/x.rs ... ignored, ignored always (not a test)
test [ui] tests/ui/non_modrs_mods_and_inline_mods/x/y/z/mod.rs ... ignored, ignored always (not a test)
test [ui] tests/ui/no_share-struct.rs ... ok
test [ui] tests/ui/no-reuse-move-arc.rs ... ok
---
test [ui] tests/ui/packed/issue-27060-2.rs ... ok
test [ui] tests/ui/overloaded/overloaded-autoderef-xcrate.rs ... ok
test [ui] tests/ui/overloaded/overloaded-calls-object-one-arg.rs ... ok
test [ui] tests/ui/packed/issue-27060.rs ... ok
test [ui] tests/ui/packed/packed-struct-borrow-element-64bit.rs ... ignored, ignored when the pointer width is 32bit ((needs `usize` to be 8-aligned to reproduce all the errors below))
test [ui] tests/ui/overloaded/overloaded-calls-object-two-args.rs ... ok
test [ui] tests/ui/overloaded/overloaded-calls-param-vtables.rs ... ok
test [ui] tests/ui/overloaded/overloaded-autoderef.rs ... ok
test [ui] tests/ui/overloaded/overloaded-index-in-field.rs ... ok
---
test [ui] tests/ui/parser/bounds-lifetime.rs ... ok
test [ui] tests/ui/parser/bounds-obj-parens.rs ... ok
test [ui] tests/ui/parser/byte-string-literals.rs ... ok
test [ui] tests/ui/parser/byte-literals.rs ... ok
test [ui] tests/ui/parser/circular_modules_hello.rs ... ignored, ignored always (this is an auxiliary file for circular-modules-main.rs)
test [ui] tests/ui/panics/runtime-switch.rs#legacy ... ok
test [ui] tests/ui/parser/class-implements-bad-trait.rs ... ok
test [ui] tests/ui/parser/closure-return-syntax.rs ... ok
test [ui] tests/ui/parser/char/whitespace-character-literal.rs ... ok
---
test [ui] tests/ui/parser/issues/issue-33413.rs ... ok
test [ui] tests/ui/parser/issues/issue-34255-1.rs ... ok
test [ui] tests/ui/parser/issues/issue-43196.rs ... ok
test [ui] tests/ui/parser/issues/issue-41155.rs ... ok
test [ui] tests/ui/parser/issues/issue-48508-aux.rs ... ignored, ignored always (Not a test. Used by issue-48508.rs)
test [ui] tests/ui/parser/issues/issue-33418.rs ... ok
test [ui] tests/ui/parser/issues/issue-43692.rs ... ok
test [ui] tests/ui/parser/issues/issue-48137-macros-cannot-interpolate-impl-items-bad-variants.rs ... ok
test [ui] tests/ui/parser/issues/issue-44406.rs ... ok
---
test [ui] tests/ui/rmeta/rmeta.rs ... ok
test [ui] tests/ui/rfcs/rfc-2005-default-binding-mode/tuple-struct.rs ... ok
test [ui] tests/ui/rfcs/rfc-2005-default-binding-mode/tuple.rs ... ok
test [ui] tests/ui/rmeta/rmeta_meta_main.rs ... ok
test [ui] tests/ui/runtime/backtrace-debuginfo-aux.rs ... ignored, ignored always (not a test, used by backtrace-debuginfo.rs to test file!())
test [ui] tests/ui/rfcs/rfc-2151-raw-identifiers/items.rs ... ok
test [ui] tests/ui/rmeta/rmeta_lib.rs ... ok
test [ui] tests/ui/rfcs/rfc-2302-self-struct-ctor.rs ... ok
test [ui] tests/ui/rfcs/rfc-2151-raw-identifiers/macros.rs ... ok
---
test [ui] tests/ui/type-alias-impl-trait/issue-60407.rs ... ok
test [ui] tests/ui/tuple/tup.rs ... ok
test [ui] tests/ui/type-alias-impl-trait/issue-60564-working.rs ... ok
test [ui] tests/ui/type-alias-impl-trait/issue-62000-associate-impl-trait-lifetimes.rs ... ok
test [ui] tests/ui/type-alias-impl-trait/issue-65918.rs ... ignored, ignored always (This now ICEs again.)
test [ui] tests/ui/type-alias-impl-trait/issue-60564.rs ... ok
test [ui] tests/ui/type-alias-impl-trait/issue-63355.rs ... ok
test [ui] tests/ui/type-alias-impl-trait/issue-65384.rs ... ok
test [ui] tests/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs ... ok
---
---- [ui] tests/ui/regions/regions-bot.rs stdout ----

error: test run failed!
status: exit status: 101
command: RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bot/a"
--- stdout -------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bot/a", waiting for result
--- stderr -------------------------------
thread 'main' panicked at 'client.read_exact(&mut header) failed with Connection reset by peer (os error 104)', src/tools/remote-test-client/src/main.rs:310:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------
