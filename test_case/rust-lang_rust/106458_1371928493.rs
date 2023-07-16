plain
Successfully built c586c113b625
Successfully tagged rust-ci:latest
Built container sha256:c586c113b6259898ba2af001944fb750075c51ad34678b38a72d79f63ae398c3
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
tidy error: No error code was found in compilation errors!
tidy error: Error code E0208 needs to have at least one UI test!
tidy error: Error code E0280 needs to have at least one UI test!
tidy error: Error code E0455 needs to have at least one UI test!
tidy error: Error code E0457 needs to have at least one UI test!
tidy error: Error code E0460 needs to have at least one UI test!
tidy error: Error code E0462 needs to have at least one UI test!
tidy error: Error code E0464 needs to have at least one UI test!
tidy error: Error code E0472 needs to have at least one UI test!
tidy error: Error code E0519 needs to have at least one UI test!
tidy error: Error code E0570 needs to have at least one UI test!
tidy error: Error code E0601 needs to have at least one UI test!
tidy error: Error code E0602 needs to have at least one UI test!
tidy error: Error code E0638 needs to have at least one UI test!
tidy error: Error code E0639 needs to have at least one UI test!
tidy error: Error code E0706 needs to have at least one UI test!
tidy error: Error code E0711 needs to have at least one UI test!
tidy error: Error code E0725 needs to have at least one UI test!
tidy error: Error code E0761 needs to have at least one UI test!
tidy error: Error code E0780 needs to have at least one UI test!
tidy error: Error code E0786 needs to have at least one UI test!
Expected a gate test for the feature 'lang_items'.
Hint: create a failing test file named 'feature-gate-lang_items.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(lang_items)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-lang_items line to the test file.
Expected a gate test for the feature 'abi_x86_interrupt'.
Hint: create a failing test file named 'feature-gate-abi_x86_interrupt.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_x86_interrupt)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_x86_interrupt line to the test file.
Expected a gate test for the feature 'auto_traits'.
Hint: create a failing test file named 'feature-gate-auto_traits.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(auto_traits)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-auto_traits line to the test file.
Expected a gate test for the feature 'return_position_impl_trait_in_trait'.
Hint: create a failing test file named 'feature-gate-return_position_impl_trait_in_trait.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(return_position_impl_trait_in_trait)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-return_position_impl_trait_in_trait line to the test file.
Expected a gate test for the feature 'register_tool'.
Hint: create a failing test file named 'feature-gate-register_tool.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(register_tool)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-register_tool line to the test file.
Expected a gate test for the feature 'cmse_nonsecure_entry'.
Hint: create a failing test file named 'feature-gate-cmse_nonsecure_entry.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cmse_nonsecure_entry)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cmse_nonsecure_entry line to the test file.
Expected a gate test for the feature 'yeet_expr'.
Hint: create a failing test file named 'feature-gate-yeet_expr.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(yeet_expr)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-yeet_expr line to the test file.
Expected a gate test for the feature 'trait_alias'.
Hint: create a failing test file named 'feature-gate-trait_alias.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(trait_alias)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-trait_alias line to the test file.
Expected a gate test for the feature 'with_negative_coherence'.
Hint: create a failing test file named 'feature-gate-with_negative_coherence.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(with_negative_coherence)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-with_negative_coherence line to the test file.
Expected a gate test for the feature 'const_fn_floating_point_arithmetic'.
Hint: create a failing test file named 'feature-gate-const_fn_floating_point_arithmetic.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_fn_floating_point_arithmetic)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_fn_floating_point_arithmetic line to the test file.
Expected a gate test for the feature 'generator_clone'.
Hint: create a failing test file named 'feature-gate-generator_clone.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generator_clone)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generator_clone line to the test file.
Expected a gate test for the feature 'capture_disjoint_fields'.
Hint: create a failing test file named 'feature-gate-capture_disjoint_fields.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(capture_disjoint_fields)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-capture_disjoint_fields line to the test file.
Expected a gate test for the feature 'const_try'.
Hint: create a failing test file named 'feature-gate-const_try.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_try)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_try line to the test file.
Expected a gate test for the feature 'generic_arg_infer'.
Hint: create a failing test file named 'feature-gate-generic_arg_infer.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generic_arg_infer)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generic_arg_infer line to the test file.
Expected a gate test for the feature 'non_exhaustive_omitted_patterns_lint'.
Hint: create a failing test file named 'feature-gate-non_exhaustive_omitted_patterns_lint.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(non_exhaustive_omitted_patterns_lint)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-non_exhaustive_omitted_patterns_lint line to the test file.
Expected a gate test for the feature 'abi_avr_interrupt'.
Hint: create a failing test file named 'feature-gate-abi_avr_interrupt.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_avr_interrupt)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_avr_interrupt line to the test file.
Expected a gate test for the feature 'dropck_eyepatch'.
Hint: create a failing test file named 'feature-gate-dropck_eyepatch.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(dropck_eyepatch)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-dropck_eyepatch line to the test file.
Expected a gate test for the feature 'must_not_suspend'.
Hint: create a failing test file named 'feature-gate-must_not_suspend.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(must_not_suspend)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-must_not_suspend line to the test file.
Expected a gate test for the feature 'optimize_attribute'.
Hint: create a failing test file named 'feature-gate-optimize_attribute.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(optimize_attribute)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-optimize_attribute line to the test file.
Expected a gate test for the feature 'trait_upcasting'.
Hint: create a failing test file named 'feature-gate-trait_upcasting.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(trait_upcasting)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-trait_upcasting line to the test file.
Expected a gate test for the feature 'rustc_attrs'.
Hint: create a failing test file named 'feature-gate-rustc_attrs.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rustc_attrs)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rustc_attrs line to the test file.
Expected a gate test for the feature 'rustdoc_missing_doc_code_examples'.
Hint: create a failing test file named 'feature-gate-rustdoc_missing_doc_code_examples.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rustdoc_missing_doc_code_examples)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rustdoc_missing_doc_code_examples line to the test file.
Expected a gate test for the feature 'avx512_target_feature'.
Hint: create a failing test file named 'feature-gate-avx512_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(avx512_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-avx512_target_feature line to the test file.
Expected a gate test for the feature 'anonymous_lifetime_in_impl_trait'.
Hint: create a failing test file named 'feature-gate-anonymous_lifetime_in_impl_trait.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(anonymous_lifetime_in_impl_trait)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-anonymous_lifetime_in_impl_trait line to the test file.
Expected a gate test for the feature 'simd_ffi'.
Hint: create a failing test file named 'feature-gate-simd_ffi.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(simd_ffi)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-simd_ffi line to the test file.
Expected a gate test for the feature 'bpf_target_feature'.
Hint: create a failing test file named 'feature-gate-bpf_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(bpf_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-bpf_target_feature line to the test file.
Expected a gate test for the feature 'collapse_debuginfo'.
Hint: create a failing test file named 'feature-gate-collapse_debuginfo.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(collapse_debuginfo)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-collapse_debuginfo line to the test file.
Expected a gate test for the feature 'impl_trait_in_fn_trait_return'.
Hint: create a failing test file named 'feature-gate-impl_trait_in_fn_trait_return.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(impl_trait_in_fn_trait_return)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-impl_trait_in_fn_trait_return line to the test file.
Expected a gate test for the feature 'fundamental'.
Hint: create a failing test file named 'feature-gate-fundamental.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(fundamental)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-fundamental line to the test file.
Expected a gate test for the feature 'abi_c_cmse_nonsecure_call'.
Hint: create a failing test file named 'feature-gate-abi_c_cmse_nonsecure_call.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_c_cmse_nonsecure_call)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_c_cmse_nonsecure_call line to the test file.
Expected a gate test for the feature 'riscv_target_feature'.
Hint: create a failing test file named 'feature-gate-riscv_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(riscv_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-riscv_target_feature line to the test file.
Expected a gate test for the feature 'arbitrary_self_types'.
Hint: create a failing test file named 'feature-gate-arbitrary_self_types.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(arbitrary_self_types)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-arbitrary_self_types line to the test file.
Expected a gate test for the feature 'deprecated_suggestion'.
Hint: create a failing test file named 'feature-gate-deprecated_suggestion.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(deprecated_suggestion)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-deprecated_suggestion line to the test file.
Expected a gate test for the feature 'rustc_allow_const_fn_unstable'.
Hint: create a failing test file named 'feature-gate-rustc_allow_const_fn_unstable.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rustc_allow_const_fn_unstable)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rustc_allow_const_fn_unstable line to the test file.
Expected a gate test for the feature 'marker_trait_attr'.
Hint: create a failing test file named 'feature-gate-marker_trait_attr.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(marker_trait_attr)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-marker_trait_attr line to the test file.
Expected a gate test for the feature 'string_deref_patterns'.
Hint: create a failing test file named 'feature-gate-string_deref_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(string_deref_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-string_deref_patterns line to the test file.
Expected a gate test for the feature 'naked_functions'.
Hint: create a failing test file named 'feature-gate-naked_functions.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(naked_functions)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-naked_functions line to the test file.
Expected a gate test for the feature 'c_variadic'.
Hint: create a failing test file named 'feature-gate-c_variadic.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(c_variadic)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-c_variadic line to the test file.
Expected a gate test for the feature 'macro_metavar_expr'.
Hint: create a failing test file named 'feature-gate-macro_metavar_expr.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(macro_metavar_expr)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-macro_metavar_expr line to the test file.
Expected a gate test for the feature 'transparent_unions'.
Hint: create a failing test file named 'feature-gate-transparent_unions.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(transparent_unions)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-transparent_unions line to the test file.
Expected a gate test for the feature 'let_chains'.
Hint: create a failing test file named 'feature-gate-let_chains.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(let_chains)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-let_chains line to the test file.
Expected a gate test for the feature 'wasm_target_feature'.
Hint: create a failing test file named 'feature-gate-wasm_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(wasm_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-wasm_target_feature line to the test file.
Expected a gate test for the feature 'no_sanitize'.
Hint: create a failing test file named 'feature-gate-no_sanitize.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(no_sanitize)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-no_sanitize line to the test file.
Expected a gate test for the feature 'type_ascription'.
Hint: create a failing test file named 'feature-gate-type_ascription.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(type_ascription)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-type_ascription line to the test file.
Expected a gate test for the feature 'abi_amdgpu_kernel'.
Hint: create a failing test file named 'feature-gate-abi_amdgpu_kernel.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_amdgpu_kernel)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_amdgpu_kernel line to the test file.
Expected a gate test for the feature 'debugger_visualizer'.
Hint: create a failing test file named 'feature-gate-debugger_visualizer.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(debugger_visualizer)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-debugger_visualizer line to the test file.
Expected a gate test for the feature 'half_open_range_patterns_in_slices'.
Hint: create a failing test file named 'feature-gate-half_open_range_patterns_in_slices.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(half_open_range_patterns_in_slices)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-half_open_range_patterns_in_slices line to the test file.
Expected a gate test for the feature 'cfg_target_has_atomic_equal_alignment'.
Hint: create a failing test file named 'feature-gate-cfg_target_has_atomic_equal_alignment.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_has_atomic_equal_alignment)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_has_atomic_equal_alignment line to the test file.
Expected a gate test for the feature 'cfg_target_thread_local'.
Hint: create a failing test file named 'feature-gate-cfg_target_thread_local.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_thread_local)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_thread_local line to the test file.
Expected a gate test for the feature 'ermsb_target_feature'.
Hint: create a failing test file named 'feature-gate-ermsb_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ermsb_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ermsb_target_feature line to the test file.
Expected a gate test for the feature 'movbe_target_feature'.
Hint: create a failing test file named 'feature-gate-movbe_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(movbe_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-movbe_target_feature line to the test file.
Expected a gate test for the feature 'if_let_guard'.
Hint: create a failing test file named 'feature-gate-if_let_guard.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(if_let_guard)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-if_let_guard line to the test file.
Expected a gate test for the feature 'aarch64_ver_target_feature'.
Hint: create a failing test file named 'feature-gate-aarch64_ver_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(aarch64_ver_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-aarch64_ver_target_feature line to the test file.
Expected a gate test for the feature 'box_patterns'.
Hint: create a failing test file named 'feature-gate-box_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(box_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-box_patterns line to the test file.
Expected a gate test for the feature 'raw_dylib'.
Hint: create a failing test file named 'feature-gate-raw_dylib.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(raw_dylib)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-raw_dylib line to the test file.
Expected a gate test for the feature 'lint_reasons'.
Hint: create a failing test file named 'feature-gate-lint_reasons.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(lint_reasons)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-lint_reasons line to the test file.
Expected a gate test for the feature 'more_qualified_paths'.
Hint: create a failing test file named 'feature-gate-more_qualified_paths.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(more_qualified_paths)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-more_qualified_paths line to the test file.
Expected a gate test for the feature 'dyn_star'.
Hint: create a failing test file named 'feature-gate-dyn_star.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(dyn_star)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-dyn_star line to the test file.
Expected a gate test for the feature 'target_feature_11'.
Hint: create a failing test file named 'feature-gate-target_feature_11.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(target_feature_11)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-target_feature_11 line to the test file.
Expected a gate test for the feature 'asm_const'.
Hint: create a failing test file named 'feature-gate-asm_const.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(asm_const)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-asm_const line to the test file.
Expected a gate test for the feature 'unix_sigpipe'.
Hint: create a failing test file named 'feature-gate-unix_sigpipe.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(unix_sigpipe)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-unix_sigpipe line to the test file.
Expected a gate test for the feature 'async_closure'.
Hint: create a failing test file named 'feature-gate-async_closure.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(async_closure)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-async_closure line to the test file.
Expected a gate test for the feature 'type_alias_impl_trait'.
Hint: create a failing test file named 'feature-gate-type_alias_impl_trait.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(type_alias_impl_trait)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-type_alias_impl_trait line to the test file.
Expected a gate test for the feature 'abi_ptx'.
Hint: create a failing test file named 'feature-gate-abi_ptx.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_ptx)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_ptx line to the test file.
Expected a gate test for the feature 'trivial_bounds'.
Hint: create a failing test file named 'feature-gate-trivial_bounds.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(trivial_bounds)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-trivial_bounds line to the test file.
Expected a gate test for the feature 'test_2018_feature'.
Hint: create a failing test file named 'feature-gate-test_2018_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(test_2018_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-test_2018_feature line to the test file.
Expected a gate test for the feature 'specialization'.
Hint: create a failing test file named 'feature-gate-specialization.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(specialization)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-specialization line to the test file.
Expected a gate test for the feature 'proc_macro_hygiene'.
Hint: create a failing test file named 'feature-gate-proc_macro_hygiene.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(proc_macro_hygiene)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-proc_macro_hygiene line to the test file.
Expected a gate test for the feature 'extended_varargs_abi_support'.
Hint: create a failing test file named 'feature-gate-extended_varargs_abi_support.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(extended_varargs_abi_support)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-extended_varargs_abi_support line to the test file.
Expected a gate test for the feature 'native_link_modifiers_as_needed'.
Hint: create a failing test file named 'feature-gate-native_link_modifiers_as_needed.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(native_link_modifiers_as_needed)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-native_link_modifiers_as_needed line to the test file.
Expected a gate test for the feature 'cfg_sanitize'.
Hint: create a failing test file named 'feature-gate-cfg_sanitize.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_sanitize)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_sanitize line to the test file.
Expected a gate test for the feature 'generic_assert'.
Hint: create a failing test file named 'feature-gate-generic_assert.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generic_assert)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generic_assert line to the test file.
Expected a gate test for the feature 'abi_vectorcall'.
Hint: create a failing test file named 'feature-gate-abi_vectorcall.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_vectorcall)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_vectorcall line to the test file.
Expected a gate test for the feature 'imported_main'.
Hint: create a failing test file named 'feature-gate-imported_main.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(imported_main)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-imported_main line to the test file.
Expected a gate test for the feature 'deprecated_safe'.
Hint: create a failing test file named 'feature-gate-deprecated_safe.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(deprecated_safe)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-deprecated_safe line to the test file.
Expected a gate test for the feature 'object_safe_for_dispatch'.
Hint: create a failing test file named 'feature-gate-object_safe_for_dispatch.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(object_safe_for_dispatch)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-object_safe_for_dispatch line to the test file.
Expected a gate test for the feature 'asm_experimental_arch'.
Hint: create a failing test file named 'feature-gate-asm_experimental_arch.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(asm_experimental_arch)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-asm_experimental_arch line to the test file.
Expected a gate test for the feature 'alloc_error_handler'.
Hint: create a failing test file named 'feature-gate-alloc_error_handler.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(alloc_error_handler)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-alloc_error_handler line to the test file.
Expected a gate test for the feature 'strict_provenance'.
Hint: create a failing test file named 'feature-gate-strict_provenance.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(strict_provenance)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-strict_provenance line to the test file.
Expected a gate test for the feature 'ffi_pure'.
