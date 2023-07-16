
$ nice ./x.py test src/tools/clippy --stage 1 --bless
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.15s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.21s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.15s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.44s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage1 tool clippy-driver (x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.15s
Building rustdoc for stage1 (x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.15s
    Finished release [optimized + debuginfo] target(s) in 0.17s
     Running unittests src/driver.rs (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/clippy_driver-0181631d583b3af7)

running 1 test
test test_arg_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/check-fmt.rs (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/check_fmt-81065ea743565b13)

running 1 test
test fmt ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/compile-test.rs (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/compile_test-c1c723f6055712e9)

running 3 tests
test rustfix_coverage_known_exceptions_accuracy ... ok
test ui_cargo_toml_metadata ... ok

running 773 tests
test [ui] ui/allow_attributes_without_reason.rs ... ok
test [ui] ui/approx_const.rs ... ok
test [ui] ui/asm_syntax.rs ... ok
test [ui] ui/as_conversions.rs ... ok
test [ui] ui/arithmetic_side_effects.rs ... ok
test [ui] ui/assertions_on_constants.rs ... ok
test [ui] ui/as_underscore.rs ... ok
test [ui] ui/absurd-extreme-comparisons.rs ... ok
test [ui] ui/almost_complete_letter_range.rs ... ok
test [ui] ui/author.rs ... ok
test [ui] ui/author/call.rs ... ok
test [ui] ui/attrs.rs ... ok
test [ui] ui/author/blocks.rs ... ok
test [ui] ui/author/issue_3849.rs ... ok
test [ui] ui/author/if.rs ... ok
test [ui] ui/author/matches.rs ... ok
test [ui] ui/assign_ops2.rs ... ok
test [ui] ui/async_yields_async.rs ... ok
test [ui] ui/assertions_on_result_states.rs ... ok
test [ui] ui/author/repeat.rs ... ok
test [ui] ui/author/struct.rs ... ok
test [ui] ui/author/loop.rs ... ok
test [ui] ui/assign_ops.rs ... ok
test [ui] ui/blanket_clippy_restriction_lints.rs ... ok
test [ui] ui/await_holding_refcell_ref.rs ... ok
test [ui] ui/bit_masks.rs ... ok
test [ui] ui/blocks_in_if_conditions_closure.rs ... ok
test [ui] ui/bind_instead_of_map.rs ... ok
test [ui] ui/blocks_in_if_conditions.rs ... ok
test [ui] ui/await_holding_lock.rs ... ok
test [ui] ui/bind_instead_of_map_multipart.rs ... ok
test [ui] ui/borrow_as_ptr.rs ... ok
test [ui] ui/bool_to_int_with_if.rs ... ok
test [ui] ui/borrow_deref_ref_unfixable.rs ... ok
test [ui] ui/borrow_box.rs ... ok
test [ui] ui/bool_assert_comparison.rs ... ok
test [ui] ui/borrow_as_ptr_no_std.rs ... ok
test [ui] ui/bool_comparison.rs ... ok
test [ui] ui/borrow_deref_ref.rs ... ok
test [ui] ui/box_collection.rs ... ok
test [ui] ui/borrow_interior_mutable_const/others.rs ... ok
test [ui] ui/borrow_interior_mutable_const/traits.rs ... ok
test [ui] ui/branches_sharing_code/false_positives.rs ... ok
test [ui] ui/borrow_interior_mutable_const/enums.rs ... ok
test [ui] ui/boxed_local.rs ... ok
test [ui] ui/branches_sharing_code/shared_at_top.rs ... ok
test [ui] ui/branches_sharing_code/shared_at_bottom.rs ... ok
test [ui] ui/bytecount.rs ... ok
test [ui] ui/builtin_type_shadow.rs ... ok
test [ui] ui/branches_sharing_code/valid_if_blocks.rs ... ok
test [ui] ui/bytes_nth.rs ... ok
test [ui] ui/branches_sharing_code/shared_at_top_and_bottom.rs ... ok
test [ui] ui/bytes_count_to_len.rs ... ok
test [ui] ui/case_sensitive_file_extension_comparisons.rs ... ok
test [ui] ui/cast_enum_constructor.rs ... ok
test [ui] ui/cast_alignment.rs ... ok
test [ui] ui/cast.rs ... ok
test [ui] ui/cast_size_32bit.rs ... ignored
test [ui] ui/cast_abs_to_unsigned.rs ... ok
test [ui] ui/cast_size.rs ... ok
test [ui] ui/cast_lossless_float.rs ... ok
test [ui] ui/cast_lossless_bool.rs ... ok
test [ui] ui/cast_lossless_integer.rs ... ok
test [ui] ui/cast_slice_different_sizes.rs ... ok
test [ui] ui/cast_ref_to_mut.rs ... ok
test [ui] ui/char_lit_as_u8.rs ... ok
test [ui] ui/cast_raw_slice_pointer_cast.rs ... ok
test [ui] ui/cfg_attr_rustfmt.rs ... ok
test [ui] ui/checked_unwrap/complex_conditionals.rs ... ok
test [ui] ui/checked_unwrap/complex_conditionals_nested.rs ... ok
test [ui] ui/char_lit_as_u8_suggestions.rs ... ok
test [ui] ui/checked_unwrap/simple_conditionals.rs ... ok
test [ui] ui/clone_on_copy_impl.rs ... ok
test [ui] ui/cmp_null.rs ... ok
test [ui] ui/cmp_nan.rs ... ok
test [ui] ui/cloned_instead_of_copied.rs ... ok
test [ui] ui/clone_on_copy.rs ... ok
test [ui] ui/checked_conversions.rs ... ok
test [ui] ui/cmp_owned/without_suggestion.rs ... ok
test [ui] ui/cmp_owned/comparison_flip.rs ... ok
test [ui] ui/cognitive_complexity_attr_used.rs ... ok
test [ui] ui/cmp_owned/asymmetric_partial_eq.rs ... ok
test [ui] ui/cmp_owned/with_suggestion.rs ... ok
test [ui] ui/collapsible_match.rs ... ok
test [ui] ui/collapsible_match2.rs ... ok
test [ui] ui/comparison_chain.rs ... ok
test [ui] ui/cognitive_complexity.rs ... ok
test [ui] ui/collapsible_else_if.rs ... ok
test [ui] ui/collapsible_if.rs ... ok
test [ui] ui/copy_iterator.rs ... ok
test [ui] ui/crashes/enum-glob-import-crate.rs ... ok
test [ui] ui/crashes/associated-constant-ice.rs ... ok
test [ui] ui/collapsible_str_replace.rs ... ok
test [ui] ui/crashes/cc_seme.rs ... ok
test [ui] ui/crashes/ice-1588.rs ... ok
test [ui] ui/comparison_to_empty.rs ... ok
test [ui] ui/crashes/ice-1969.rs ... ok
test [ui] ui/crashes/ice-2760.rs ... ok
test [ui] ui/crashes/ice-1782.rs ... ok
test [ui] ui/crashes/ice-2727.rs ... ok
test [ui] ui/crashes/ice-2594.rs ... ok
test [ui] ui/crashes/ice-2499.rs ... ok
test [ui] ui/crashes/ice-2862.rs ... ok
test [ui] ui/crashes/ice-360.rs ... ok
test [ui] ui/crashes/ice-2865.rs ... ok
test [ui] ui/crashes/ice-2774.rs ... ok
test [ui] ui/crashes/ice-3462.rs ... ok
test [ui] ui/crashes/ice-3151.rs ... ok
test [ui] ui/crashes/ice-3717.rs ... ok
test [ui] ui/crashes/ice-3891.rs ... ok
test [ui] ui/crashes/ice-3747.rs ... ok
test [ui] ui/crashes/ice-4545.rs ... ok
test [ui] ui/crashes/ice-4121.rs ... ok
test [ui] ui/crashes/ice-4579.rs ... ok
test [ui] ui/crashes/ice-3969.rs ... ok
test [ui] ui/crashes/ice-4727.rs ... ok
test [ui] ui/crashes/ice-4760.rs ... ok
test [ui] ui/crashes/ice-4671.rs ... ok
test [ui] ui/crashes/ice-5207.rs ... ok
test [ui] ui/crashes/ice-4775.rs ... ok
test [ui] ui/crashes/ice-5497.rs ... ok
test [ui] ui/crashes/ice-4968.rs ... ok
test [ui] ui/crashes/ice-5238.rs ... ok
test [ui] ui/crashes/ice-5223.rs ... ok
test [ui] ui/crashes/ice-5389.rs ... ok
test [ui] ui/crashes/ice-5579.rs ... ok
test [ui] ui/crashes/ice-5835.rs ... ok
test [ui] ui/crashes/ice-5872.rs ... ok
test [ui] ui/crashes/ice-6139.rs ... ok
test [ui] ui/crashes/ice-5944.rs ... ok
test [ui] ui/crashes/ice-6153.rs ... ok
test [ui] ui/crashes/ice-6179.rs ... ok
test [ui] ui/crashes/ice-6250.rs ... ok
test [ui] ui/crashes/ice-6256.rs ... ok
test [ui] ui/crashes/ice-6255.rs ... ok
test [ui] ui/crashes/ice-6254.rs ... ok
test [ui] ui/crashes/ice-6332.rs ... ok
test [ui] ui/crashes/ice-6539.rs ... ok
test [ui] ui/crashes/ice-6792.rs ... ok
test [ui] ui/crashes/ice-6251.rs ... ok
test [ui] ui/crashes/ice-6793.rs ... ok
test [ui] ui/crashes/ice-6840.rs ... ok
test [ui] ui/crashes/ice-700.rs ... ok
test [ui] ui/crashes/ice-7012.rs ... ok
test [ui] ui/crashes/ice-7126.rs ... ok
test [ui] ui/crashes/ice-7169.rs ... ok
test [ui] ui/crashes/ice-6252.rs ... ok
test [ui] ui/crashes/ice-7231.rs ... ok
test [ui] ui/crashes/ice-7340.rs ... ok
test [ui] ui/crashes/ice-7410.rs ... ok
test [ui] ui/crashes/ice-7868.rs ... ok
test [ui] ui/crashes/ice-7272.rs ... ok
test [ui] ui/crashes/ice-7869.rs ... ok
test [ui] ui/crashes/ice-7423.rs ... ok
test [ui] ui/crashes/ice-7934.rs ... ok
test [ui] ui/crashes/ice-8250.rs ... ok
test [ui] ui/crashes/ice-8386.rs ... ok
test [ui] ui/crashes/ice-8821.rs ... ok
test [ui] ui/crashes/ice-9238.rs ... ok
test [ui] ui/crashes/ice-9041.rs ... ok
test [ui] ui/crashes/ice-9242.rs ... ok
test [ui] ui/crashes/ice-8850.rs ... ok
test [ui] ui/crashes/ice-9414.rs ... ok
test [ui] ui/crashes/ice-9405.rs ... ok
test [ui] ui/crashes/ice-96721.rs ... ok
test [ui] ui/crashes/ice-8681.rs ... ok
test [ui] ui/crashes/ice_exacte_size.rs ... ok
test [ui] ui/crashes/implements-trait.rs ... ok
test [ui] ui/crashes/if_same_then_else.rs ... ok
test [ui] ui/crashes/inherent_impl.rs ... ok
test [ui] ui/crashes/issues_loop_mut_cond.rs ... ok
test [ui] ui/crashes/ice-3741.rs ... ok
test [ui] ui/crashes/match_same_arms_const.rs ... ok
test [ui] ui/crashes/needless_lifetimes_impl_trait.rs ... ok
test [ui] ui/crashes/mut_mut_macro.rs ... ok
test [ui] ui/crashes/returns.rs ... ok
test [ui] ui/crashes/issue-825.rs ... ok
test [ui] ui/crashes/needless_borrow_fp.rs ... ok
test [ui] ui/crashes/third-party/conf_allowlisted.rs ... ok
test [ui] ui/crashes/shadow.rs ... ok
test [ui] ui/crashes/trivial_bounds.rs ... ok
test [ui] ui/crashes/regressions.rs ... ok
test [ui] ui/crashes/single-match-else.rs ... ok
test [ui] ui/crate_level_checks/no_std_main_recursion.rs ... ok
test [ui] ui/crate_level_checks/entrypoint_recursion.rs ... ok
test [ui] ui/crate_level_checks/no_std_swap.rs ... ok
test [ui] ui/crate_level_checks/std_main_recursion.rs ... ok
test [ui] ui/crashes/used_underscore_binding_macro.rs ... ok
test [ui] ui/declare_interior_mutable_const/enums.rs ... ok
test [ui] ui/crate_in_macro_def.rs ... ok
test [ui] ui/create_dir.rs ... ok
test [ui] ui/decimal_literal_representation.rs ... ok
test [ui] ui/def_id_nocore.rs ... ok
test [ui] ui/declare_interior_mutable_const/others.rs ... ok
test [ui] ui/debug_assert_with_mut_call.rs ... ok
test [ui] ui/declare_interior_mutable_const/traits.rs ... ok
test [ui] ui/default_union_representation.rs ... ok
test [ui] ui/default_instead_of_iter_empty.rs ... ok
test [ui] ui/deprecated_old.rs ... ok
test [ui] ui/deprecated.rs ... ok
test [ui] ui/dbg_macro.rs ... ok
test [ui] ui/deref_addrof_macro.rs ... ok
test [ui] ui/deref_addrof_double_trigger.rs ... ok
test [ui] ui/derivable_impls.rs ... ok
test [ui] ui/default_numeric_fallback_i32.rs ... ok
test [ui] ui/deref_addrof.rs ... ok
test [ui] ui/derive_hash_xor_eq.rs ... ok
test [ui] ui/default_numeric_fallback_f64.rs ... ok
test [ui] ui/derive.rs ... ok
test [ui] ui/deref_by_slicing.rs ... ok
test [ui] ui/disallowed_script_idents.rs ... ok
test [ui] ui/disallowed_names.rs ... ok
test [ui] ui/diverging_sub_expression.rs ... ok
test [ui] ui/doc/issue_1832.rs ... ok
test [ui] ui/doc/issue_902.rs ... ok
test [ui] ui/derive_ord_xor_partial_ord.rs ... ok
test [ui] ui/doc/unbalanced_ticks.rs ... ok
test [ui] ui/doc_link_with_quotes.rs ... ok
test [ui] ui/double_neg.rs ... ok
test [ui] ui/doc_errors.rs ... ok
test [ui] ui/double_must_use.rs ... ok
test [ui] ui/double_comparison.rs ... ok
test [ui] ui/doc_unsafe.rs ... ok
test [ui] ui/doc/doc-fixable.rs ... ok
test [ui] ui/derive_partial_eq_without_eq.rs ... ok
test [ui] ui/drop_non_drop.rs ... ok
test [ui] ui/drop_forget_copy.rs ... ok
test [ui] ui/double_parens.rs ... ok
test [ui] ui/duplicate_underscore_argument.rs ... ok
test [ui] ui/drop_ref.rs ... ok
test [ui] ui/empty_enum.rs ... ok
test [ui] ui/else_if_without_else.rs ... ok
test [ui] ui/empty_enum_without_never_type.rs ... ok
test [ui] ui/duration_subsec.rs ... ok
test [ui] ui/empty_loop_no_std.rs ... ok
test [ui] ui/empty_drop.rs ... ok
test [ui] ui/empty_loop.rs ... ok
test [ui] ui/empty_structs_with_brackets.rs ... ok
test [ui] ui/entry_btree.rs ... ok
test [ui] ui/enum_clike_unportable_variant.rs ... ok
test [ui] ui/entry.rs ... ok
test [ui] ui/enum_variants.rs ... ok
test [ui] ui/entry_with_else.rs ... ok
test [ui] ui/eprint_with_newline.rs ... ok
test [ui] ui/erasing_op.rs ... ok
test [ui] ui/eq_op_macros.rs ... ok
test [ui] ui/enum_glob_use.rs ... ok
test [ui] ui/eq_op.rs ... ok
test [ui] ui/err_expect.rs ... ok
test [ui] ui/exhaustive_items.rs ... ok
test [ui] ui/excessive_precision.rs ... ok
test [ui] ui/exit1.rs ... ok
test [ui] ui/exit2.rs ... ok
test [ui] ui/exit3.rs ... ok
test [ui] ui/expect.rs ... ok
test [ui] ui/equatable_if_let.rs ... ok
test [ui] ui/expect_tool_lint_rfc_2383.rs ... ok
test [ui] ui/explicit_counter_loop.rs ... ok
test [ui] ui/expect_fun_call.rs ... ok
test [ui] ui/extend_with_drain.rs ... ok
test [ui] ui/default_trait_access.rs ... ok
test [ui] ui/eta.rs ... ok
test [ui] ui/fallible_impl_from.rs ... ok
test [ui] ui/explicit_write.rs ... ok
test [ui] ui/explicit_deref_methods.rs ... ok
test [ui] ui/explicit_auto_deref.rs ... ok
test [ui] ui/filetype_is_file.rs ... ok
test [ui] ui/filter_map_next.rs ... ok
test [ui] ui/find_map.rs ... ok
test [ui] ui/filter_map_identity.rs ... ok
test [ui] ui/filter_map_next_fixable.rs ... ok
test [ui] ui/flat_map_identity.rs ... ok
test [ui] ui/flat_map_option.rs ... ok
test [ui] ui/float_arithmetic.rs ... ok
test [ui] ui/float_cmp.rs ... ok
test [ui] ui/float_cmp_const.rs ... ok
test [ui] ui/float_equality_without_abs.rs ... ok
test [ui] ui/floating_point_exp.rs ... ok
test [ui] ui/floating_point_hypot.rs ... ok
test [ui] ui/floating_point_abs.rs ... ok
test [ui] ui/floating_point_logbase.rs ... ok
test [ui] ui/floating_point_log.rs ... ok
test [ui] ui/floating_point_powi.rs ... ok
test [ui] ui/fn_address_comparisons.rs ... ok
test [ui] ui/floating_point_mul_add.rs ... ok
test [ui] ui/fn_to_numeric_cast_32bit.rs ... ignored
test [ui] ui/floating_point_powf.rs ... ok
test [ui] ui/floating_point_rad.rs ... ok
test [ui] ui/fn_params_excessive_bools.rs ... ok
test [ui] ui/empty_line_after_outer_attribute.rs ... ok
test [ui] ui/fn_to_numeric_cast.rs ... ok
test [ui] ui/for_loop_unfixable.rs ... ok
test [ui] ui/for_kv_map.rs ... ok
test [ui] ui/fn_to_numeric_cast_any.rs ... ok
test [ui] ui/forget_non_drop.rs ... ok
test [ui] ui/for_loops_over_fallibles.rs ... ok
test [ui] ui/forget_ref.rs ... ok
test [ui] ui/format_push_string.rs ... ok
test [ui] ui/formatting.rs ... ok
test [ui] ui/format_args_unfixable.rs ... ok
test [ui] ui/from_over_into.rs ... ok
test [ui] ui/extra_unused_lifetimes.rs ... ok
test [ui] ui/from_str_radix_10.rs ... ok
test [ui] ui/format.rs ... ok
test [ui] ui/functions.rs ... ok
test [ui] ui/for_loop_fixable.rs ... ok
test [ui] ui/format_args.rs ... ok
test [ui] ui/from_iter_instead_of_collect.rs ... ok
test [ui] ui/future_not_send.rs ... ok
test [ui] ui/functions_maxlines.rs ... ok
test [ui] ui/if_let_mutex.rs ... ok
test [ui] ui/field_reassign_with_default.rs ... ok
test [ui] ui/get_last_with_len.rs ... ok
test [ui] ui/get_first.rs ... ok
test [ui] ui/if_not_else.rs ... ok
test [ui] ui/ifs_same_cond.rs ... ok
test [ui] ui/if_same_then_else2.rs ... ok
test [ui] ui/if_same_then_else.rs ... ok
test [ui] ui/if_then_some_else_none.rs ... ok
test [ui] ui/impl.rs ... ok
test [ui] ui/get_unwrap.rs ... ok
test [ui] ui/identity_op.rs ... ok
test [ui] ui/index_refutable_slice/slice_indexing_in_macro.rs ... ok
test [ui] ui/inconsistent_digit_grouping.rs ... ok
test [ui] ui/implicit_hasher.rs ... ok
test [ui] ui/implicit_clone.rs ... ok
test [ui] ui/implicit_return.rs ... ok
test [ui] ui/indexing_slicing_index.rs ... FAILED
test [ui] ui/index_refutable_slice/if_let_slice_binding.rs ... ok
test [ui] ui/inconsistent_struct_constructor.rs ... ok
test [ui] ui/indexing_slicing_slice.rs ... ok
test [ui] ui/implicit_saturating_sub.rs ... ok
test [ui] ui/inherent_to_string.rs ... ok
test [ui] ui/inefficient_to_string.rs ... ok
test [ui] ui/inspect_for_each.rs ... ok
test [ui] ui/infallible_destructuring_match.rs ... ok
test [ui] ui/inline_fn_without_body.rs ... ok
test [ui] ui/infinite_iter.rs ... ok
test [ui] ui/infinite_loop.rs ... ok
test [ui] ui/int_plus_one.rs ... ok
test [ui] ui/integer_division.rs ... ok
test [ui] ui/integer_arithmetic.rs ... ok
test [ui] ui/issue-3145.rs ... ok
test [ui] ui/invalid_utf8_in_unchecked.rs ... ok
test [ui] ui/invalid_upcast_comparisons.rs ... ok
test [ui] ui/issue-7447.rs ... ok
test [ui] ui/item_after_statement.rs ... ok
test [ui] ui/is_digit_ascii_radix.rs ... ok
test [ui] ui/invalid_null_ptr_usage.rs ... ok
test [ui] ui/issue_4266.rs ... ok
test [ui] ui/into_iter_on_ref.rs ... ok
test [ui] ui/issue_2356.rs ... ok
test [ui] ui/iter_not_returning_iterator.rs ... ok
test [ui] ui/iter_cloned_collect.rs ... ok
test [ui] ui/iter_next_slice.rs ... ok
test [ui] ui/iter_nth_zero.rs ... ok
test [ui] ui/iter_nth.rs ... ok
test [ui] ui/iter_skip_next_unfixable.rs ... ok
test [ui] ui/iterator_step_by_zero.rs ... ok
test [ui] ui/iter_on_empty_collections.rs ... ok
test [ui] ui/iter_on_single_items.rs ... ok
test [ui] ui/large_digit_groups.rs ... ok
test [ui] ui/iter_overeager_cloned.rs ... ok
test [ui] ui/large_enum_variant.rs ... ok
test [ui] ui/iter_count.rs ... ok
test [ui] ui/iter_with_drain.rs ... ok
test [ui] ui/large_stack_arrays.rs ... ok
test [ui] ui/large_types_passed_by_value.rs ... ok
test [ui] ui/iter_skip_next.rs ... ok
test [ui] ui/len_without_is_empty.rs ... ok
test [ui] ui/let_and_return.rs ... ok
test [ui] ui/let_underscore_drop.rs ... ok
test [ui] ui/len_zero_ranges.rs ... ok
test [ui] ui/let_if_seq.rs ... ok
test [ui] ui/len_zero.rs ... ok
test [ui] ui/literals.rs ... ok
test [ui] ui/linkedlist.rs ... ok
test [ui] ui/let_underscore_lock.rs ... ok
test [ui] ui/let_underscore_must_use.rs ... ok
test [ui] ui/lossy_float_literal.rs ... ok
test [ui] ui/manual_bits.rs ... ok
test [ui] ui/manual_async_fn.rs ... ok
test [ui] ui/let_unit.rs ... ok
test [ui] ui/manual_find.rs ... ok
test [ui] ui/manual_flatten.rs ... ok
test [ui] ui/manual_assert.rs ... ok
test [ui] ui/manual_filter_map.rs ... ok
test [ui] ui/manual_instant_elapsed.rs ... ok
test [ui] ui/manual_find_fixable.rs ... ok
test [ui] ui/manual_find_map.rs ... ok
test [ui] ui/manual_memcpy/with_loop_counters.rs ... ok
test [ui] ui/manual_map_option_2.rs ... ok
test [ui] ui/manual_memcpy/without_loop_counters.rs ... ok
test [ui] ui/manual_non_exhaustive_enum.rs ... ok
test [ui] ui/manual_non_exhaustive_struct.rs ... ok
test [ui] ui/manual_map_option.rs ... ok
test [ui] ui/manual_ok_or.rs ... ok
test [ui] ui/manual_saturating_arithmetic.rs ... ok
test [ui] ui/manual_rem_euclid.rs ... ok
test [ui] ui/manual_str_repeat.rs ... ok
test [ui] ui/manual_strip.rs ... ok
test [ui] ui/manual_string_new.rs ... ok
test [ui] ui/many_single_char_names.rs ... ok
test [ui] ui/macro_use_imports_expect.rs ... ok
test [ui] ui/manual_split_once.rs ... ok
test [ui] ui/map_err.rs ... ok
test [ui] ui/map_flatten.rs ... ok
test [ui] ui/map_collect_result_unit.rs ... ok
test [ui] ui/manual_unwrap_or.rs ... ok
test [ui] ui/map_clone.rs ... ok
test [ui] ui/map_unit_fn.rs ... ok
test [ui] ui/map_flatten_fixable.rs ... ok
test [ui] ui/map_identity.rs ... ok
test [ui] ui/map_unwrap_or.rs ... ok
test [ui] ui/match_as_ref.rs ... ok
test [ui] ui/match_bool.rs ... ok
test [ui] ui/manual_retain.rs ... ok
test [ui] ui/match_on_vec_items.rs ... ok
test [ui] ui/match_expr_like_matches_macro.rs ... ok
test [ui] ui/match_overlapping_arm.rs ... ok
test [ui] ui/match_same_arms.rs ... ok
test [ui] ui/map_unwrap_or_fixable.rs ... ok
test [ui] ui/match_same_arms2.rs ... ok
test [ui] ui/match_ref_pats.rs ... ok
test [ui] ui/match_result_ok.rs ... ok
test [ui] ui/mem_forget.rs ... ok
test [ui] ui/match_single_binding2.rs ... ok
test [ui] ui/match_wildcard_for_single_variants.rs ... ok
test [ui] ui/match_str_case_mismatch.rs ... ok
test [ui] ui/match_wild_err_arm.rs ... ok
test [ui] ui/match_single_binding.rs ... ok
test [ui] ui/mem_replace_macro.rs ... ok
test [ui] ui/min_max.rs ... ok
test [ui] ui/min_rust_version_invalid_attr.rs ... ok
test [ui] ui/min_rust_version_multiple_inner_attr.rs ... ok
test [ui] ui/mem_replace.rs ... ok
test [ui] ui/methods.rs ... ok
test [ui] ui/methods_fixable.rs ... ok
test [ui] ui/min_rust_version_outer_attr.rs ... ok
test [ui] ui/min_rust_version_no_patch.rs ... ok
test [ui] ui/min_rust_version_attr.rs ... ok
test [ui] ui/macro_use_imports.rs ... ok
test [ui] ui/mismatching_type_param_order.rs ... ok
test [ui] ui/mismatched_target_os_non_unix.rs ... ok
test [ui] ui/missing_const_for_fn/could_be_const.rs ... ok
test [ui] ui/mismatched_target_os_unix.rs ... ok
test [ui] ui/missing_doc_crate.rs ... ok
test [ui] ui/missing_doc_crate_missing.rs ... ok
test [ui] ui/missing_inline_executable.rs ... ok
test [ui] ui/missing_inline.rs ... ok
test [ui] ui/missing_inline_proc_macro.rs ... ok
test [ui] ui/missing_panics_doc.rs ... ok
test [ui] ui/missing_spin_loop.rs ... ok
test [ui] ui/missing_spin_loop_no_std.rs ... ok
test [ui] ui/mixed_read_write_in_expression.rs ... ok
test [ui] ui/module_inception.rs ... ok
test [ui] ui/module_name_repetitions.rs ... ok
test [ui] ui/modulo_arithmetic_float.rs ... ok
test [ui] ui/modulo_arithmetic_integral.rs ... ok
test [ui] ui/modulo_arithmetic_integral_const.rs ... ok
test [ui] ui/modulo_one.rs ... ok
test [ui] ui/multi_assignments.rs ... ok
test [ui] ui/mut_from_ref.rs ... ok
test [ui] ui/mut_key.rs ... ok
test [ui] ui/must_use_candidates.rs ... ok
test [ui] ui/missing_doc.rs ... ok
test [ui] ui/must_use_unit.rs ... ok
test [ui] ui/mut_mut.rs ... ok
test [ui] ui/mut_range_bound.rs ... ok
test [ui] ui/missing_doc_impl.rs ... ok
test [ui] ui/missing_const_for_fn/cant_be_const.rs ... ok
test [ui] ui/mut_reference.rs ... ok
test [ui] ui/mutex_atomic.rs ... ok
test [ui] ui/mut_mutex_lock.rs ... ok
test [ui] ui/needless_bool/simple.rs ... ok
test [ui] ui/needless_arbitrary_self_type.rs ... ok
test [ui] ui/needless_bitwise_bool.rs ... ok
test [ui] ui/needless_borrow_pat.rs ... ok
test [ui] ui/needless_bool/fixable.rs ... ok
test [ui] ui/needless_borrowed_ref.rs ... ok
test [ui] ui/needless_borrow.rs ... ok
test [ui] ui/needless_doc_main.rs ... ok
test [ui] ui/needless_collect.rs ... ok
test [ui] ui/needless_continue.rs ... ok
test [ui] ui/needless_for_each_unfixable.rs ... ok
test [ui] ui/needless_collect_indirect.rs ... ok
test [ui] ui/needless_lifetimes.rs ... ok
test [ui] ui/needless_match.rs ... ok
test [ui] ui/needless_for_each_fixable.rs ... ok
test [ui] ui/needless_late_init.rs ... ok
test [ui] ui/needless_option_as_deref.rs ... ok
test [ui] ui/needless_option_take.rs ... ok
test [ui] ui/mistyped_literal_suffix.rs ... ok
test [ui] ui/needless_parens_on_range_literals.rs ... ok
test [ui] ui/needless_pass_by_value_proc_macro.rs ... ok
test [ui] ui/needless_pass_by_value.rs ... ok
test [ui] ui/needless_range_loop.rs ... ok
test [ui] ui/needless_update.rs ... ok
test [ui] ui/needless_question_mark.rs ... ok
test [ui] ui/neg_cmp_op_on_partial_ord.rs ... ok
test [ui] ui/needless_splitn.rs ... ok
test [ui] ui/needless_range_loop2.rs ... ok
test [ui] ui/neg_multiply.rs ... ok
test [ui] ui/never_loop.rs ... ok
test [ui] ui/new_without_default.rs ... ok
test [ui] ui/new_ret_no_self.rs ... ok
test [ui] ui/no_effect_replace.rs ... ok
test [ui] ui/needless_return.rs ... ok
test [ui] ui/no_effect.rs ... ok
test [ui] ui/non_send_fields_in_send_ty.rs ... ok
test [ui] ui/non_expressive_names.rs ... ok
test [ui] ui/nonminimal_bool.rs ... ok
test [ui] ui/large_const_arrays.rs ... ok
test [ui] ui/octal_escapes.rs ... ok
test [ui] ui/non_octal_unix_permissions.rs ... ok
test [ui] ui/numbered_fields.rs ... ok
test [ui] ui/obfuscated_if_else.rs ... ok
test [ui] ui/only_used_in_recursion2.rs ... ok
test [ui] ui/ok_expect.rs ... ok
test [ui] ui/open_options.rs ... ok
test [ui] ui/only_used_in_recursion.rs ... ok
test [ui] ui/nonminimal_bool_methods.rs ... ok
test [ui] ui/op_ref.rs ... ok
test [ui] ui/option_env_unwrap.rs ... ok
test [ui] ui/needless_arbitrary_self_type_unfixable.rs ... ok
test [ui] ui/option_filter_map.rs ... ok
test [ui] ui/option_map_or_none.rs ... ok
test [ui] ui/option_map_unit_fn_unfixable.rs ... ok
test [ui] ui/out_of_bounds_indexing/issue-3102.rs ... ok
test [ui] ui/option_as_ref_deref.rs ... ok
test [ui] ui/option_map_unit_fn_fixable.rs ... ok
test [ui] ui/out_of_bounds_indexing/simple.rs ... ok
test [ui] ui/option_if_let_else.rs ... ok
test [ui] ui/option_option.rs ... ok
test [ui] ui/overflow_check_conditional.rs ... ok
test [ui] ui/overly_complex_bool_expr.rs ... ok
test [ui] ui/or_then_unwrap.rs ... ok
test [ui] ui/panic_in_result_fn.rs ... ok
test [ui] ui/panic_in_result_fn_debug_assertions.rs ... ok
test [ui] ui/panic_in_result_fn_assertions.rs ... ok
test [ui] ui/partialeq_ne_impl.rs ... ok
test [ui] ui/path_buf_push_overwrite.rs ... ok
test [ui] ui/pattern_type_mismatch/pattern_alternatives.rs ... ok
test [ui] ui/pattern_type_mismatch/mutability.rs ... ok
test [ui] ui/panicking_macros.rs ... ok
test [ui] ui/pattern_type_mismatch/pattern_tuples.rs ... ok
test [ui] ui/pattern_type_mismatch/pattern_structs.rs ... ok
test [ui] ui/partialeq_to_none.rs ... ok
test [ui] ui/pattern_type_mismatch/syntax.rs ... ok
test [ui] ui/or_fun_call.rs ... ok
test [ui] ui/print_stderr.rs ... ok
test [ui] ui/print.rs ... ok
test [ui] ui/print_in_format_impl.rs ... ok
test [ui] ui/print_stdout_build_script.rs ... ok
test [ui] ui/patterns.rs ... ok
test [ui] ui/print_literal.rs ... ok
test [ui] ui/print_with_newline.rs ... ok
test [ui] ui/proc_macro.rs ... ok
test [ui] ui/precedence.rs ... ok
test [ui] ui/println_empty_string.rs ... ok
test [ui] ui/pub_use.rs ... ok
test [ui] ui/ptr_eq.rs ... ok
test [ui] ui/ptr_offset_with_cast.rs ... ok
test [ui] ui/ptr_arg.rs ... ok
test [ui] ui/range.rs ... ok
test [ui] ui/positional_named_format_parameters.rs ... ok
test [ui] ui/rc_buffer_redefined_string.rs ... ok
test [ui] ui/ptr_as_ptr.rs ... ok
test [ui] ui/rc_buffer.rs ... ok
test [ui] ui/rc_buffer_arc.rs ... ok
test [ui] ui/range_contains.rs ... ok
test [ui] ui/question_mark.rs ... ok
test [ui] ui/range_plus_minus_one.rs ... ok
test [ui] ui/rc_mutex.rs ... ok
test [ui] ui/rc_clone_in_vec_init/arc.rs ... ok
test [ui] ui/rc_clone_in_vec_init/rc.rs ... ok
test [ui] ui/rc_clone_in_vec_init/weak.rs ... ok
test [ui] ui/read_zero_byte_vec.rs ... ok
test [ui] ui/redundant_closure_call_early.rs ... ok
test [ui] ui/redundant_allocation.rs ... ok
test [ui] ui/redundant_closure_call_late.rs ... ok
test [ui] ui/redundant_allocation_fixable.rs ... ok
test [ui] ui/recursive_format_impl.rs ... ok
test [ui] ui/redundant_closure_call_fixable.rs ... ok
test [ui] ui/redundant_else.rs ... ok
test [ui] ui/redundant_field_names.rs ... ok
test [ui] ui/redundant_pattern_matching_drop_order.rs ... ok
test [ui] ui/redundant_pattern_matching_poll.rs ... ok
test [ui] ui/redundant_pattern_matching_option.rs ... ok
test [ui] ui/redundant_clone.rs ... ok
test [ui] ui/redundant_pattern_matching_ipaddr.rs ... ok
test [ui] ui/redundant_static_lifetimes_multiple.rs ... ok
test [ui] ui/redundant_pub_crate.rs ... ok
test [ui] ui/ref_option_ref.rs ... ok
test [ui] ui/redundant_slicing.rs ... ok
test [ui] ui/ref_binding_to_reference.rs ... ok
test [ui] ui/redundant_pattern_matching_result.rs ... ok
test [ui] ui/redundant_static_lifetimes.rs ... ok
test [ui] ui/regex.rs ... ok
test [ui] ui/renamed_builtin_attr.rs ... ok
test [ui] ui/repl_uninit.rs ... ok
test [ui] ui/rest_pat_in_fully_bound_structs.rs ... ok
test [ui] ui/rename.rs ... ok
test [ui] ui/result_large_err.rs ... ok
test [ui] ui/result_unit_error.rs ... ok
test [ui] ui/repeat_once.rs ... ok
test [ui] ui/return_self_not_must_use.rs ... ok
test [ui] ui/result_map_unit_fn_unfixable.rs ... ok
test [ui] ui/result_map_or_into_option.rs ... ok
test [ui] ui/reversed_empty_ranges_loops_unfixable.rs ... ok
test [ui] ui/same_name_method.rs ... ok
test [ui] ui/reversed_empty_ranges_unfixable.rs ... ok
test [ui] ui/same_functions_in_if_condition.rs ... ok
test [ui] ui/result_map_unit_fn_fixable.rs ... ok
test [ui] ui/reversed_empty_ranges_fixable.rs ... ok
test [ui] ui/same_item_push.rs ... ok
test [ui] ui/self_assignment.rs ... ok
test [ui] ui/self_named_constructors.rs ... ok
test [ui] ui/reversed_empty_ranges_loops_fixable.rs ... ok
test [ui] ui/semicolon_if_nothing_returned.rs ... ok
test [ui] ui/serde.rs ... ok
test [ui] ui/search_is_some.rs ... ok
test [ui] ui/shadow.rs ... ok
test [ui] ui/should_impl_trait/corner_cases.rs ... ok
test [ui] ui/should_impl_trait/method_list_1.rs ... ok
test [ui] ui/short_circuit_statement.rs ... ok
test [ui] ui/should_impl_trait/method_list_2.rs ... ok
test [ui] ui/single_char_lifetime_names.rs ... ok
test [ui] ui/single_component_path_imports_macro.rs ... ok
test [ui] ui/similar_names.rs ... ok
test [ui] ui/single_char_add_str.rs ... ok
test [ui] ui/single_component_path_imports.rs ... ok
test [ui] ui/single_component_path_imports_nested_first.rs ... ok
test [ui] ui/single_component_path_imports_self_after.rs ... ok
test [ui] ui/single_component_path_imports_self_before.rs ... ok
test [ui] ui/single_char_pattern.rs ... ok
test [ui] ui/search_is_some_fixable_none.rs ... ok
test [ui] ui/size_of_in_element_count/functions.rs ... ok
test [ui] ui/size_of_in_element_count/expressions.rs ... ok
test [ui] ui/significant_drop_in_scrutinee.rs ... ok
test [ui] ui/search_is_some_fixable_some.rs ... ok
test [ui] ui/single_match.rs ... ok
test [ui] ui/slow_vector_initialization.rs ... ok
test [ui] ui/std_instead_of_core.rs ... ok
test [ui] ui/str_to_string.rs ... ok
test [ui] ui/skip_while_next.rs ... ok
test [ui] ui/single_element_loop.rs ... ok
test [ui] ui/starts_ends_with.rs ... ok
test [ui] ui/string_from_utf8_as_bytes.rs ... ok
test [ui] ui/string_slice.rs ... ok
test [ui] ui/string_add.rs ... ok
test [ui] ui/string_extend.rs ... ok
test [ui] ui/stable_sort_primitive.rs ... ok
test [ui] ui/string_lit_as_bytes.rs ... ok
test [ui] ui/string_to_string.rs ... ok
test [ui] ui/struct_excessive_bools.rs ... ok
test [ui] ui/string_add_assign.rs ... ok
test [ui] ui/suspicious_splitn.rs ... ok
test [ui] ui/suspicious_map.rs ... ok
test [ui] ui/suspicious_arithmetic_impl.rs ... ok
test [ui] ui/strlen_on_c_strings.rs ... ok
test [ui] ui/suspicious_to_owned.rs ... ok
test [ui] ui/suspicious_unary_op_formatting.rs ... ok
test [ui] ui/swap_ptr_to_ref_unfixable.rs ... ok
test [ui] ui/swap_ptr_to_ref.rs ... ok
test [ui] ui/suspicious_operation_groupings.rs ... ok
test [ui] ui/temporary_assignment.rs ... ok
test [ui] ui/tabs_in_doc_comments.rs ... ok
test [ui] ui/to_digit_is_some.rs ... ok
test [ui] ui/trailing_zeros.rs ... ok
test [ui] ui/single_match_else.rs ... ok
test [ui] ui/toplevel_ref_arg_non_rustfix.rs ... ok
test [ui] ui/transmute_32bit.rs ... ignored
test [ui] ui/swap.rs ... ok
test [ui] ui/trailing_empty_array.rs ... ok
test [ui] ui/transmute_64bit.rs ... ok
test [ui] ui/transmute_float_to_int.rs ... ok
test [ui] ui/trait_duplication_in_bounds_unfixable.rs ... ok
test [ui] ui/toplevel_ref_arg.rs ... ok
test [ui] ui/transmute.rs ... ok
test [ui] ui/transmute_ptr_to_ptr.rs ... ok
test [ui] ui/transmute_collection.rs ... ok
test [ui] ui/trait_duplication_in_bounds.rs ... ok
test [ui] ui/transmuting_null.rs ... ok
test [ui] ui/transmute_undefined_repr.rs ... ok
test [ui] ui/ty_fn_sig.rs ... ok
test [ui] ui/transmute_ptr_to_ref.rs ... ok
test [ui] ui/trim_split_whitespace.rs ... ok
test [ui] ui/transmutes_expressible_as_ptr_casts.rs ... ok
test [ui] ui/type_complexity.rs ... ok
test [ui] ui/trivially_copy_pass_by_ref.rs ... ok
test [ui] ui/types.rs ... ok
test [ui] ui/undropped_manually_drops.rs ... ok
test [ui] ui/suspicious_else_formatting.rs ... ok
test [ui] ui/uninit.rs ... ok
test [ui] ui/type_repetition_in_bounds.rs ... ok
test [ui] ui/uninit_vec.rs ... ok
test [ui] ui/unit_cmp.rs ... ok
test [ui] ui/unit_hash.rs ... ok
test [ui] ui/try_err.rs ... ok
test [ui] ui/unknown_attribute.rs ... ok
test [ui] ui/unit_return_expecting_ord.rs ... ok
test [ui] ui/unicode.rs ... ok
test [ui] ui/unit_arg_empty_blocks.rs ... ok
test [ui] ui/unnecessary_find_map.rs ... ok
test [ui] ui/unknown_clippy_lints.rs ... ok
test [ui] ui/unnecessary_clone.rs ... ok
test [ui] ui/unnecessary_filter_map.rs ... ok
test [ui] ui/unnecessary_cast.rs ... ok
test [ui] ui/unnecessary_lazy_eval_unfixable.rs ... ok
test [ui] ui/unnecessary_fold.rs ... ok
test [ui] ui/unnecessary_join.rs ... ok
test [ui] ui/unnecessary_operation.rs ... ok
test [ui] ui/unnecessary_self_imports.rs ... ok
test [ui] ui/unnecessary_iter_cloned.rs ... ok
test [ui] ui/unnecessary_owned_empty_strings.rs ... ok
test [ui] ui/unneeded_field_pattern.rs ... ok
test [ui] ui/unnecessary_lazy_eval.rs ... ok
test [ui] ui/unnecessary_wraps.rs ... ok
test [ui] ui/undocumented_unsafe_blocks.rs ... ok
test [ui] ui/unneeded_wildcard_pattern.rs ... ok
test [ui] ui/unnested_or_patterns2.rs ... ok
test [ui] ui/unnested_or_patterns.rs ... ok
test [ui] ui/unit_arg.rs ... ok
test [ui] ui/unsafe_removed_from_name.rs ... ok
test [ui] ui/unreadable_literal.rs ... ok
test [ui] ui/unsafe_derive_deserialize.rs ... ok
test [ui] ui/unused_async.rs ... ok
test [ui] ui/unused_peekable.rs ... ok
test [ui] ui/unused_rounding.rs ... ok
test [ui] ui/unnecessary_sort_by.rs ... ok
test [ui] ui/unused_self.rs ... ok
test [ui] ui/unwrap.rs ... ok
test [ui] ui/unwrap_expect_used.rs ... ok
test [ui] ui/unused_unit.rs ... ok
test [ui] ui/unused_io_amount.rs ... ok
test [ui] ui/unwrap_in_result.rs ... ok
test [ui] ui/unwrap_or.rs ... ok
test [ui] ui/upper_case_acronyms.rs ... ok
test [ui] ui/unnecessary_to_owned.rs ... ok
test [ui] ui/useful_asref.rs ... ok
test [ui] ui/use_self_trait.rs ... ok
test [ui] ui/unwrap_or_else_default.rs ... ok
test [ui] ui/useless_conversion_try.rs ... ok
test [ui] ui/useless_asref.rs ... ok
test [ui] ui/useless_conversion.rs ... ok
test [ui] ui/vec_box_sized.rs ... ok
test [ui] ui/vec.rs ... ok
test [ui] ui/vec_init_then_push.rs ... ok
test [ui] ui/vec_resize_to_zero.rs ... ok
test [ui] ui/verbose_file_reads.rs ... ok
test [ui] ui/vtable_address_comparisons.rs ... ok
test [ui] ui/while_let_loop.rs ... ok
test [ui] ui/wild_in_or_pats.rs ... ok
test [ui] ui/write_literal.rs ... ok
test [ui] ui/write_literal_2.rs ... ok
test [ui] ui/wildcard_enum_match_arm.rs ... ok
test [ui] ui/wildcard_imports.rs ... ok
test [ui] ui/write_with_newline.rs ... ok
test [ui] ui/wrong_self_convention2.rs ... ok
test [ui] ui/wrong_self_convention.rs ... ok
test [ui] ui/writeln_empty_string.rs ... ok
test [ui] ui/while_let_on_iterator.rs ... ok
test [ui] ui/zero_div_zero.rs ... ok
test [ui] ui/used_underscore_binding.rs ... ok
test [ui] ui/zero_offset.rs ... ok
test [ui] ui/wrong_self_conventions_mut.rs ... ok
test [ui] ui/zero_sized_hashmap_values.rs ... ok
test [ui] ui/zero_sized_btreemap_values.rs ... ok
test [ui] ui/zero_ptr.rs ... ok
test [ui] ui/unseparated_prefix_literals.rs ... ok
test [ui] ui/useless_attribute.rs ... ok
test [ui] ui/use_self.rs ... ok

failures:

failures:
    [ui] ui/indexing_slicing_index.rs

test result: FAILED. 769 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 29.43s

test compile_test ... FAILED

failures:

---- compile_test stdout ----
diff of stderr:

 error[E0080]: evaluation of `main::{constant#3}` failed
   --> $DIR/indexing_slicing_index.rs:31:14
    |
 LL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.
    |              ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
 
 error[E0080]: erroneous constant used
   --> $DIR/indexing_slicing_index.rs:31:5
    |
 LL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.
    |     ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:22:5
    |
 LL |     x[index];
    |     ^^^^^^^^
    |
    = note: `-D clippy::indexing-slicing` implied by `-D warnings`
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:38:5
    |
 LL |     v[0];
    |     ^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:39:5
    |
 LL |     v[10];
    |     ^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:40:5
    |
 LL |     v[1 << 3];
    |     ^^^^^^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:46:5
    |
 LL |     v[N];
    |     ^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 
 error: indexing may panic
   --> $DIR/indexing_slicing_index.rs:47:5
    |
 LL |     v[M];
    |     ^^^^
    |
    = help: consider using `.get(n)` or `.get_mut(n)` instead
 
-error: aborting due to 8 previous errors
+error[E0080]: evaluation of constant value failed
+  --> $DIR/indexing_slicing_index.rs:10:24
+   |
+LL | const REF_ERR: &i32 = &ARR[idx4()]; // Ok, let rustc handle const contexts.
+   |                        ^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 4
+
+error: aborting due to 9 previous errors
 
 For more information about this error, try `rustc --explain E0080`.
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args indexing_slicing_index.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/indexing_slicing_index.rs" "-L" "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps" "--extern" "clippy_lints=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-1aef6ab2f0513d58.rlib" "--extern" "quote=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libquote-859855a643e27817.rlib" "--extern" "syn=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-18cbc48dd11933c4.rlib" "--extern" "serde=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-7edfcb84b7bb160c.rlib" "--extern" "itertools=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-6e4bc39f6fffd954.rlib" "--extern" "parking_lot=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-b129a68ce9b9c578.rlib" "--extern" "futures=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2afee9067b7048eb.rlib" "--extern" "regex=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-df6592a83722f72e.rlib" "--extern" "rustc_semver=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-78d8497d58c0ecb0.rlib" "--extern" "clippy_utils=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-0a9b1b27729454e1.rlib" "--extern" "serde_derive=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps/libserde_derive-97104e4129a028f5.so" "--extern" "derive_new=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps/libderive_new-5ada41d8d6356a45.so" "--extern" "if_chain=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-39d0f0f44e4ae2e3.rlib" "--extern" "tokio=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-b31cbb18dbe479d4.rlib" "--edition=2021" "-L" "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/test/ui/indexing_slicing_index.stage-id.aux"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{"message":"evaluation of `main::{constant#3}` failed","code":{"code":"E0080","explanation":"A constant value failed to get evaluated.\n\nErroneous code example:\n\n