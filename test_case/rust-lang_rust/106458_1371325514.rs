plain
Successfully built 7bf997140032
Successfully tagged rust-ci:latest
Built container sha256:7bf99714003270e46cb8f54447e2a5453378caa90791979837c8a760b75b9830
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
tidy check
tidy: Skipping binary file check, read-only filesystem
* 633 error codes
* highest error code: E0791
Expected a gate test for the feature 'half_open_range_patterns_in_slices'.
Hint: create a failing test file named 'feature-gate-half_open_range_patterns_in_slices.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(half_open_range_patterns_in_slices)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-half_open_range_patterns_in_slices line to the test file.
Expected a gate test for the feature 'cfg_sanitize'.
Hint: create a failing test file named 'feature-gate-cfg_sanitize.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_sanitize)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_sanitize line to the test file.
Expected a gate test for the feature 'fn_align'.
Hint: create a failing test file named 'feature-gate-fn_align.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(fn_align)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-fn_align line to the test file.
Expected a gate test for the feature 'precise_pointer_size_matching'.
Hint: create a failing test file named 'feature-gate-precise_pointer_size_matching.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(precise_pointer_size_matching)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-precise_pointer_size_matching line to the test file.
Expected a gate test for the feature 'movbe_target_feature'.
Hint: create a failing test file named 'feature-gate-movbe_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(movbe_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-movbe_target_feature line to the test file.
Expected a gate test for the feature 'dropck_eyepatch'.
Hint: create a failing test file named 'feature-gate-dropck_eyepatch.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(dropck_eyepatch)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-dropck_eyepatch line to the test file.
Expected a gate test for the feature 'string_deref_patterns'.
Hint: create a failing test file named 'feature-gate-string_deref_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(string_deref_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-string_deref_patterns line to the test file.
Expected a gate test for the feature 'const_fn_floating_point_arithmetic'.
Hint: create a failing test file named 'feature-gate-const_fn_floating_point_arithmetic.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_fn_floating_point_arithmetic)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_fn_floating_point_arithmetic line to the test file.
Expected a gate test for the feature 'specialization'.
Hint: create a failing test file named 'feature-gate-specialization.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(specialization)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-specialization line to the test file.
Expected a gate test for the feature 'const_refs_to_cell'.
Hint: create a failing test file named 'feature-gate-const_refs_to_cell.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_refs_to_cell)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_refs_to_cell line to the test file.
Expected a gate test for the feature 'platform_intrinsics'.
Hint: create a failing test file named 'feature-gate-platform_intrinsics.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(platform_intrinsics)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-platform_intrinsics line to the test file.
Expected a gate test for the feature 'generic_assert'.
Hint: create a failing test file named 'feature-gate-generic_assert.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generic_assert)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generic_assert line to the test file.
Expected a gate test for the feature 'rustc_allow_const_fn_unstable'.
Hint: create a failing test file named 'feature-gate-rustc_allow_const_fn_unstable.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rustc_allow_const_fn_unstable)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rustc_allow_const_fn_unstable line to the test file.
Expected a gate test for the feature 'sse4a_target_feature'.
Hint: create a failing test file named 'feature-gate-sse4a_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(sse4a_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-sse4a_target_feature line to the test file.
Expected a gate test for the feature 'repr_simd'.
Hint: create a failing test file named 'feature-gate-repr_simd.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(repr_simd)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-repr_simd line to the test file.
Expected a gate test for the feature 'inline_const'.
Hint: create a failing test file named 'feature-gate-inline_const.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(inline_const)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-inline_const line to the test file.
Expected a gate test for the feature 'cfg_target_has_atomic'.
Hint: create a failing test file named 'feature-gate-cfg_target_has_atomic.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_has_atomic)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_has_atomic line to the test file.
Expected a gate test for the feature 'doc_auto_cfg'.
Hint: create a failing test file named 'feature-gate-doc_auto_cfg.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(doc_auto_cfg)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-doc_auto_cfg line to the test file.
Expected a gate test for the feature 'associated_type_defaults'.
Hint: create a failing test file named 'feature-gate-associated_type_defaults.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(associated_type_defaults)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-associated_type_defaults line to the test file.
Expected a gate test for the feature 'abi_vectorcall'.
Hint: create a failing test file named 'feature-gate-abi_vectorcall.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_vectorcall)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_vectorcall line to the test file.
Expected a gate test for the feature 'unsized_locals'.
Hint: create a failing test file named 'feature-gate-unsized_locals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(unsized_locals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-unsized_locals line to the test file.
Expected a gate test for the feature 'const_eval_limit'.
Hint: create a failing test file named 'feature-gate-const_eval_limit.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_eval_limit)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_eval_limit line to the test file.
Expected a gate test for the feature 'allow_internal_unsafe'.
Hint: create a failing test file named 'feature-gate-allow_internal_unsafe.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(allow_internal_unsafe)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-allow_internal_unsafe line to the test file.
Expected a gate test for the feature 'target_feature_11'.
Hint: create a failing test file named 'feature-gate-target_feature_11.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(target_feature_11)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-target_feature_11 line to the test file.
Expected a gate test for the feature 'extern_types'.
Hint: create a failing test file named 'feature-gate-extern_types.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(extern_types)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-extern_types line to the test file.
Expected a gate test for the feature 'transparent_unions'.
Hint: create a failing test file named 'feature-gate-transparent_unions.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(transparent_unions)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-transparent_unions line to the test file.
Expected a gate test for the feature 'more_qualified_paths'.
Hint: create a failing test file named 'feature-gate-more_qualified_paths.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(more_qualified_paths)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-more_qualified_paths line to the test file.
Expected a gate test for the feature 'let_chains'.
Hint: create a failing test file named 'feature-gate-let_chains.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(let_chains)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-let_chains line to the test file.
Expected a gate test for the feature 'wasm_abi'.
Hint: create a failing test file named 'feature-gate-wasm_abi.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(wasm_abi)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-wasm_abi line to the test file.
Expected a gate test for the feature 'min_specialization'.
Hint: create a failing test file named 'feature-gate-min_specialization.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(min_specialization)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-min_specialization line to the test file.
Expected a gate test for the feature 'f16c_target_feature'.
Hint: create a failing test file named 'feature-gate-f16c_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(f16c_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-f16c_target_feature line to the test file.
Expected a gate test for the feature 'rust_cold_cc'.
Hint: create a failing test file named 'feature-gate-rust_cold_cc.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rust_cold_cc)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rust_cold_cc line to the test file.
Expected a gate test for the feature 'exhaustive_patterns'.
Hint: create a failing test file named 'feature-gate-exhaustive_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(exhaustive_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-exhaustive_patterns line to the test file.
Expected a gate test for the feature 'abi_efiapi'.
Hint: create a failing test file named 'feature-gate-abi_efiapi.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_efiapi)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_efiapi line to the test file.
Expected a gate test for the feature 'linkage'.
Hint: create a failing test file named 'feature-gate-linkage.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(linkage)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-linkage line to the test file.
Expected a gate test for the feature 'needs_panic_runtime'.
Hint: create a failing test file named 'feature-gate-needs_panic_runtime.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(needs_panic_runtime)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-needs_panic_runtime line to the test file.
Expected a gate test for the feature 'asm_unwind'.
Hint: create a failing test file named 'feature-gate-asm_unwind.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(asm_unwind)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-asm_unwind line to the test file.
Expected a gate test for the feature 'rtm_target_feature'.
Hint: create a failing test file named 'feature-gate-rtm_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rtm_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rtm_target_feature line to the test file.
Expected a gate test for the feature 'type_changing_struct_update'.
Hint: create a failing test file named 'feature-gate-type_changing_struct_update.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(type_changing_struct_update)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-type_changing_struct_update line to the test file.
Expected a gate test for the feature 'decl_macro'.
Hint: create a failing test file named 'feature-gate-decl_macro.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(decl_macro)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-decl_macro line to the test file.
Expected a gate test for the feature 'ffi_returns_twice'.
Hint: create a failing test file named 'feature-gate-ffi_returns_twice.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ffi_returns_twice)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ffi_returns_twice line to the test file.
Expected a gate test for the feature 'abi_ptx'.
Hint: create a failing test file named 'feature-gate-abi_ptx.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_ptx)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_ptx line to the test file.
Expected a gate test for the feature 'anonymous_lifetime_in_impl_trait'.
Hint: create a failing test file named 'feature-gate-anonymous_lifetime_in_impl_trait.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(anonymous_lifetime_in_impl_trait)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-anonymous_lifetime_in_impl_trait line to the test file.
Expected a gate test for the feature 'deprecated_safe'.
Hint: create a failing test file named 'feature-gate-deprecated_safe.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(deprecated_safe)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-deprecated_safe line to the test file.
Expected a gate test for the feature 'impl_trait_in_fn_trait_return'.
Hint: create a failing test file named 'feature-gate-impl_trait_in_fn_trait_return.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(impl_trait_in_fn_trait_return)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-impl_trait_in_fn_trait_return line to the test file.
Expected a gate test for the feature 'staged_api'.
Hint: create a failing test file named 'feature-gate-staged_api.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(staged_api)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-staged_api line to the test file.
Expected a gate test for the feature 'test_unstable_lint'.
Hint: create a failing test file named 'feature-gate-test_unstable_lint.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(test_unstable_lint)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-test_unstable_lint line to the test file.
Expected a gate test for the feature 'thread_local'.
Hint: create a failing test file named 'feature-gate-thread_local.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(thread_local)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-thread_local line to the test file.
Expected a gate test for the feature 'unsafe_pin_internals'.
Hint: create a failing test file named 'feature-gate-unsafe_pin_internals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(unsafe_pin_internals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-unsafe_pin_internals line to the test file.
Expected a gate test for the feature 'alloc_error_handler'.
Hint: create a failing test file named 'feature-gate-alloc_error_handler.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(alloc_error_handler)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-alloc_error_handler line to the test file.
Expected a gate test for the feature 'rustdoc_internals'.
Hint: create a failing test file named 'feature-gate-rustdoc_internals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rustdoc_internals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rustdoc_internals line to the test file.
Expected a gate test for the feature 'closure_lifetime_binder'.
Hint: create a failing test file named 'feature-gate-closure_lifetime_binder.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(closure_lifetime_binder)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-closure_lifetime_binder line to the test file.
Expected a gate test for the feature 'generator_clone'.
Hint: create a failing test file named 'feature-gate-generator_clone.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generator_clone)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generator_clone line to the test file.
Expected a gate test for the feature 'closure_track_caller'.
Hint: create a failing test file named 'feature-gate-closure_track_caller.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(closure_track_caller)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-closure_track_caller line to the test file.
Expected a gate test for the feature 'const_trait_impl'.
Hint: create a failing test file named 'feature-gate-const_trait_impl.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_trait_impl)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_trait_impl line to the test file.
Expected a gate test for the feature 'doc_notable_trait'.
Hint: create a failing test file named 'feature-gate-doc_notable_trait.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(doc_notable_trait)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-doc_notable_trait line to the test file.
Expected a gate test for the feature 'ermsb_target_feature'.
Hint: create a failing test file named 'feature-gate-ermsb_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ermsb_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ermsb_target_feature line to the test file.
Expected a gate test for the feature 'plugin'.
Hint: create a failing test file named 'feature-gate-plugin.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(plugin)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-plugin line to the test file.
Expected a gate test for the feature 'abi_x86_interrupt'.
Hint: create a failing test file named 'feature-gate-abi_x86_interrupt.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_x86_interrupt)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_x86_interrupt line to the test file.
Expected a gate test for the feature 'const_mut_refs'.
Hint: create a failing test file named 'feature-gate-const_mut_refs.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_mut_refs)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_mut_refs line to the test file.
Expected a gate test for the feature 'allocator_internals'.
Hint: create a failing test file named 'feature-gate-allocator_internals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(allocator_internals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-allocator_internals line to the test file.
Expected a gate test for the feature 'macro_metavar_expr'.
Hint: create a failing test file named 'feature-gate-macro_metavar_expr.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(macro_metavar_expr)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-macro_metavar_expr line to the test file.
Expected a gate test for the feature 'trivial_bounds'.
Hint: create a failing test file named 'feature-gate-trivial_bounds.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(trivial_bounds)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-trivial_bounds line to the test file.
Expected a gate test for the feature 'capture_disjoint_fields'.
Hint: create a failing test file named 'feature-gate-capture_disjoint_fields.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(capture_disjoint_fields)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-capture_disjoint_fields line to the test file.
Expected a gate test for the feature 'const_for'.
Hint: create a failing test file named 'feature-gate-const_for.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_for)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_for line to the test file.
Expected a gate test for the feature 'tbm_target_feature'.
Hint: create a failing test file named 'feature-gate-tbm_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(tbm_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-tbm_target_feature line to the test file.
Expected a gate test for the feature 'default_type_parameter_fallback'.
Hint: create a failing test file named 'feature-gate-default_type_parameter_fallback.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(default_type_parameter_fallback)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-default_type_parameter_fallback line to the test file.
Expected a gate test for the feature 'native_link_modifiers_as_needed'.
Hint: create a failing test file named 'feature-gate-native_link_modifiers_as_needed.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(native_link_modifiers_as_needed)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-native_link_modifiers_as_needed line to the test file.
Expected a gate test for the feature 'c_variadic'.
Hint: create a failing test file named 'feature-gate-c_variadic.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(c_variadic)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-c_variadic line to the test file.
Expected a gate test for the feature 'const_async_blocks'.
Hint: create a failing test file named 'feature-gate-const_async_blocks.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_async_blocks)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_async_blocks line to the test file.
Expected a gate test for the feature 'custom_test_frameworks'.
Hint: create a failing test file named 'feature-gate-custom_test_frameworks.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(custom_test_frameworks)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-custom_test_frameworks line to the test file.
Expected a gate test for the feature 'register_tool'.
Hint: create a failing test file named 'feature-gate-register_tool.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(register_tool)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-register_tool line to the test file.
Expected a gate test for the feature 'deprecated_suggestion'.
Hint: create a failing test file named 'feature-gate-deprecated_suggestion.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(deprecated_suggestion)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-deprecated_suggestion line to the test file.
Expected a gate test for the feature 'intrinsics'.
Hint: create a failing test file named 'feature-gate-intrinsics.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(intrinsics)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-intrinsics line to the test file.
Expected a gate test for the feature 'cfg_target_thread_local'.
Hint: create a failing test file named 'feature-gate-cfg_target_thread_local.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_thread_local)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_thread_local line to the test file.
Expected a gate test for the feature 'doc_cfg_hide'.
Hint: create a failing test file named 'feature-gate-doc_cfg_hide.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(doc_cfg_hide)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-doc_cfg_hide line to the test file.
Expected a gate test for the feature 'riscv_target_feature'.
Hint: create a failing test file named 'feature-gate-riscv_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(riscv_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-riscv_target_feature line to the test file.
Expected a gate test for the feature 'generic_arg_infer'.
Hint: create a failing test file named 'feature-gate-generic_arg_infer.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generic_arg_infer)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generic_arg_infer line to the test file.
Expected a gate test for the feature 'doc_cfg'.
Hint: create a failing test file named 'feature-gate-doc_cfg.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(doc_cfg)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-doc_cfg line to the test file.
Expected a gate test for the feature 'ffi_pure'.
Hint: create a failing test file named 'feature-gate-ffi_pure.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ffi_pure)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ffi_pure line to the test file.
Expected a gate test for the feature 'must_not_suspend'.
Hint: create a failing test file named 'feature-gate-must_not_suspend.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(must_not_suspend)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-must_not_suspend line to the test file.
Expected a gate test for the feature 'ffi_const'.
Hint: create a failing test file named 'feature-gate-ffi_const.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ffi_const)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ffi_const line to the test file.
Expected a gate test for the feature 'extended_varargs_abi_support'.
Hint: create a failing test file named 'feature-gate-extended_varargs_abi_support.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(extended_varargs_abi_support)]`.
