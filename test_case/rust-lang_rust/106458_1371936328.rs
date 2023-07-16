plain
Successfully built caec12610575
Successfully tagged rust-ci:latest
Built container sha256:caec12610575b1b4bb5cd55a3a0bbc015998d11d05bb38dc234a146b11b6bdba
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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 16.53s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: /checkout/library/alloc/tests/rc.rs:207: line longer than 100 chars
tidy error: /checkout/library/alloc/tests/arc.rs:211: line longer than 100 chars
* highest error code: E0791
* highest error code: E0791
tidy error: /checkout/library/std/tests/run-time-detect.rs:105: line longer than 100 chars
tidy error: /checkout/library/std/tests/run-time-detect.rs:142: line longer than 100 chars
tidy error: /checkout/src/tools/rust-demangler/tests/lib.rs:7: line longer than 100 chars
tidy error: /checkout/src/tools/rust-demangler/tests/lib.rs:29: line longer than 100 chars
tidy error: /checkout/src/tools/rust-demangler/tests/lib.rs:30: line longer than 100 chars
tidy error: /checkout/src/tools/rust-demangler/tests/lib.rs:51: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/int_macros.rs:348: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/int_macros.rs:350: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/int_macros.rs:361: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/int_macros.rs:363: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/ops.rs:15: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/ops.rs:210: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:67: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:83: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:88: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:93: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:98: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:104: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:109: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:114: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:119: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:124: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:129: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:134: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:139: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:144: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:149: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:159: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:170: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/dec2flt/parse.rs:175: line longer than 100 chars
tidy error: /checkout/library/core/tests/num/int_log.rs:4: line longer than 100 chars
tidy error: /checkout/library/core/tests/slice.rs:2464: line longer than 100 chars
tidy error: /checkout/library/core/tests/slice.rs:2548: line longer than 100 chars
tidy error: /checkout/library/core/tests/slice.rs:2556: line longer than 100 chars
Expected a gate test for the feature 'let_chains'.
Hint: create a failing test file named 'feature-gate-let_chains.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(let_chains)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-let_chains line to the test file.
Expected a gate test for the feature 'transparent_unions'.
Hint: create a failing test file named 'feature-gate-transparent_unions.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(transparent_unions)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-transparent_unions line to the test file.
Expected a gate test for the feature 'arbitrary_self_types'.
Hint: create a failing test file named 'feature-gate-arbitrary_self_types.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(arbitrary_self_types)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-arbitrary_self_types line to the test file.
Expected a gate test for the feature 'const_async_blocks'.
Hint: create a failing test file named 'feature-gate-const_async_blocks.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_async_blocks)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_async_blocks line to the test file.
Expected a gate test for the feature 'adt_const_params'.
Hint: create a failing test file named 'feature-gate-adt_const_params.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(adt_const_params)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-adt_const_params line to the test file.
Expected a gate test for the feature 'never_type'.
Hint: create a failing test file named 'feature-gate-never_type.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(never_type)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-never_type line to the test file.
Expected a gate test for the feature 'platform_intrinsics'.
Hint: create a failing test file named 'feature-gate-platform_intrinsics.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(platform_intrinsics)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-platform_intrinsics line to the test file.
Expected a gate test for the feature 'panic_runtime'.
Hint: create a failing test file named 'feature-gate-panic_runtime.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(panic_runtime)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-panic_runtime line to the test file.
Expected a gate test for the feature 'raw_ref_op'.
Hint: create a failing test file named 'feature-gate-raw_ref_op.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(raw_ref_op)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-raw_ref_op line to the test file.
Expected a gate test for the feature 'dyn_star'.
Hint: create a failing test file named 'feature-gate-dyn_star.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(dyn_star)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-dyn_star line to the test file.
Expected a gate test for the feature 'test_2018_feature'.
Hint: create a failing test file named 'feature-gate-test_2018_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(test_2018_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-test_2018_feature line to the test file.
Expected a gate test for the feature 'cfg_sanitize'.
Hint: create a failing test file named 'feature-gate-cfg_sanitize.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_sanitize)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_sanitize line to the test file.
Expected a gate test for the feature 'allow_internal_unstable'.
Hint: create a failing test file named 'feature-gate-allow_internal_unstable.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(allow_internal_unstable)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-allow_internal_unstable line to the test file.
Expected a gate test for the feature 'cfg_target_compact'.
Hint: create a failing test file named 'feature-gate-cfg_target_compact.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_compact)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_compact line to the test file.
Expected a gate test for the feature 'test_unstable_lint'.
Hint: create a failing test file named 'feature-gate-test_unstable_lint.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(test_unstable_lint)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-test_unstable_lint line to the test file.
Expected a gate test for the feature 'generic_associated_types_extended'.
Hint: create a failing test file named 'feature-gate-generic_associated_types_extended.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generic_associated_types_extended)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generic_associated_types_extended line to the test file.
Expected a gate test for the feature 'allow_internal_unsafe'.
Hint: create a failing test file named 'feature-gate-allow_internal_unsafe.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(allow_internal_unsafe)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-allow_internal_unsafe line to the test file.
Expected a gate test for the feature 'default_type_parameter_fallback'.
Hint: create a failing test file named 'feature-gate-default_type_parameter_fallback.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(default_type_parameter_fallback)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-default_type_parameter_fallback line to the test file.
Expected a gate test for the feature 'cmse_nonsecure_entry'.
Hint: create a failing test file named 'feature-gate-cmse_nonsecure_entry.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cmse_nonsecure_entry)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cmse_nonsecure_entry line to the test file.
Expected a gate test for the feature 'deprecated_safe'.
Hint: create a failing test file named 'feature-gate-deprecated_safe.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(deprecated_safe)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-deprecated_safe line to the test file.
Expected a gate test for the feature 'extern_types'.
Hint: create a failing test file named 'feature-gate-extern_types.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(extern_types)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-extern_types line to the test file.
Expected a gate test for the feature 'closure_track_caller'.
Hint: create a failing test file named 'feature-gate-closure_track_caller.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(closure_track_caller)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-closure_track_caller line to the test file.
Expected a gate test for the feature 'cfg_version'.
Hint: create a failing test file named 'feature-gate-cfg_version.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_version)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_version line to the test file.
Expected a gate test for the feature 'needs_panic_runtime'.
Hint: create a failing test file named 'feature-gate-needs_panic_runtime.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(needs_panic_runtime)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-needs_panic_runtime line to the test file.
Expected a gate test for the feature 'rustdoc_missing_doc_code_examples'.
Hint: create a failing test file named 'feature-gate-rustdoc_missing_doc_code_examples.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rustdoc_missing_doc_code_examples)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rustdoc_missing_doc_code_examples line to the test file.
Expected a gate test for the feature 'abi_unadjusted'.
Hint: create a failing test file named 'feature-gate-abi_unadjusted.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_unadjusted)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_unadjusted line to the test file.
Expected a gate test for the feature 'c_unwind'.
Hint: create a failing test file named 'feature-gate-c_unwind.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(c_unwind)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-c_unwind line to the test file.
Expected a gate test for the feature 'used_with_arg'.
Hint: create a failing test file named 'feature-gate-used_with_arg.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(used_with_arg)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-used_with_arg line to the test file.
Expected a gate test for the feature 'large_assignments'.
Hint: create a failing test file named 'feature-gate-large_assignments.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(large_assignments)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-large_assignments line to the test file.
Expected a gate test for the feature 'repr_simd'.
Hint: create a failing test file named 'feature-gate-repr_simd.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(repr_simd)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-repr_simd line to the test file.
Expected a gate test for the feature 'rust_cold_cc'.
Hint: create a failing test file named 'feature-gate-rust_cold_cc.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(rust_cold_cc)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-rust_cold_cc line to the test file.
Expected a gate test for the feature 'box_syntax'.
Hint: create a failing test file named 'feature-gate-box_syntax.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(box_syntax)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-box_syntax line to the test file.
Expected a gate test for the feature 'with_negative_coherence'.
Hint: create a failing test file named 'feature-gate-with_negative_coherence.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(with_negative_coherence)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-with_negative_coherence line to the test file.
Expected a gate test for the feature 'register_tool'.
Hint: create a failing test file named 'feature-gate-register_tool.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(register_tool)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-register_tool line to the test file.
Expected a gate test for the feature 'ffi_pure'.
Hint: create a failing test file named 'feature-gate-ffi_pure.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ffi_pure)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ffi_pure line to the test file.
Expected a gate test for the feature 'half_open_range_patterns_in_slices'.
Hint: create a failing test file named 'feature-gate-half_open_range_patterns_in_slices.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(half_open_range_patterns_in_slices)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-half_open_range_patterns_in_slices line to the test file.
Expected a gate test for the feature 'unsafe_pin_internals'.
Hint: create a failing test file named 'feature-gate-unsafe_pin_internals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(unsafe_pin_internals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-unsafe_pin_internals line to the test file.
Expected a gate test for the feature 'no_sanitize'.
Hint: create a failing test file named 'feature-gate-no_sanitize.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(no_sanitize)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-no_sanitize line to the test file.
Expected a gate test for the feature 'asm_const'.
Hint: create a failing test file named 'feature-gate-asm_const.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(asm_const)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-asm_const line to the test file.
Expected a gate test for the feature 'trait_upcasting'.
Hint: create a failing test file named 'feature-gate-trait_upcasting.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(trait_upcasting)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-trait_upcasting line to the test file.
Expected a gate test for the feature 'profiler_runtime'.
Hint: create a failing test file named 'feature-gate-profiler_runtime.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(profiler_runtime)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-profiler_runtime line to the test file.
Expected a gate test for the feature 'marker_trait_attr'.
Hint: create a failing test file named 'feature-gate-marker_trait_attr.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(marker_trait_attr)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-marker_trait_attr line to the test file.
Expected a gate test for the feature 'if_let_guard'.
Hint: create a failing test file named 'feature-gate-if_let_guard.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(if_let_guard)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-if_let_guard line to the test file.
Expected a gate test for the feature 'generators'.
Hint: create a failing test file named 'feature-gate-generators.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(generators)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-generators line to the test file.
Expected a gate test for the feature 'box_patterns'.
Hint: create a failing test file named 'feature-gate-box_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(box_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-box_patterns line to the test file.
Expected a gate test for the feature 'inherent_associated_types'.
Hint: create a failing test file named 'feature-gate-inherent_associated_types.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(inherent_associated_types)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-inherent_associated_types line to the test file.
Expected a gate test for the feature 'unboxed_closures'.
Hint: create a failing test file named 'feature-gate-unboxed_closures.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(unboxed_closures)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-unboxed_closures line to the test file.
Expected a gate test for the feature 'allocator_internals'.
Hint: create a failing test file named 'feature-gate-allocator_internals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(allocator_internals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-allocator_internals line to the test file.
Expected a gate test for the feature 'anonymous_lifetime_in_impl_trait'.
Hint: create a failing test file named 'feature-gate-anonymous_lifetime_in_impl_trait.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(anonymous_lifetime_in_impl_trait)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-anonymous_lifetime_in_impl_trait line to the test file.
Expected a gate test for the feature 'aarch64_ver_target_feature'.
Hint: create a failing test file named 'feature-gate-aarch64_ver_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(aarch64_ver_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-aarch64_ver_target_feature line to the test file.
Expected a gate test for the feature 'const_extern_fn'.
Hint: create a failing test file named 'feature-gate-const_extern_fn.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_extern_fn)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_extern_fn line to the test file.
Expected a gate test for the feature 'doc_cfg'.
Hint: create a failing test file named 'feature-gate-doc_cfg.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(doc_cfg)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-doc_cfg line to the test file.
Expected a gate test for the feature 'cmpxchg16b_target_feature'.
Hint: create a failing test file named 'feature-gate-cmpxchg16b_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cmpxchg16b_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cmpxchg16b_target_feature line to the test file.
Expected a gate test for the feature 'associated_type_defaults'.
Hint: create a failing test file named 'feature-gate-associated_type_defaults.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(associated_type_defaults)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-associated_type_defaults line to the test file.
Expected a gate test for the feature 'doc_cfg_hide'.
Hint: create a failing test file named 'feature-gate-doc_cfg_hide.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(doc_cfg_hide)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-doc_cfg_hide line to the test file.
Expected a gate test for the feature 'no_core'.
Hint: create a failing test file named 'feature-gate-no_core.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(no_core)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-no_core line to the test file.
Expected a gate test for the feature 'proc_macro_hygiene'.
Hint: create a failing test file named 'feature-gate-proc_macro_hygiene.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(proc_macro_hygiene)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-proc_macro_hygiene line to the test file.
Expected a gate test for the feature 'thread_local'.
Hint: create a failing test file named 'feature-gate-thread_local.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(thread_local)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-thread_local line to the test file.
Expected a gate test for the feature 'wasm_target_feature'.
Hint: create a failing test file named 'feature-gate-wasm_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(wasm_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-wasm_target_feature line to the test file.
Expected a gate test for the feature 'const_for'.
Hint: create a failing test file named 'feature-gate-const_for.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(const_for)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-const_for line to the test file.
Expected a gate test for the feature 'exclusive_range_pattern'.
Hint: create a failing test file named 'feature-gate-exclusive_range_pattern.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(exclusive_range_pattern)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-exclusive_range_pattern line to the test file.
Expected a gate test for the feature 'async_closure'.
Hint: create a failing test file named 'feature-gate-async_closure.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(async_closure)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-async_closure line to the test file.
Expected a gate test for the feature 'extended_varargs_abi_support'.
Hint: create a failing test file named 'feature-gate-extended_varargs_abi_support.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(extended_varargs_abi_support)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-extended_varargs_abi_support line to the test file.
Expected a gate test for the feature 'abi_avr_interrupt'.
Hint: create a failing test file named 'feature-gate-abi_avr_interrupt.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(abi_avr_interrupt)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-abi_avr_interrupt line to the test file.
Expected a gate test for the feature 'custom_test_frameworks'.
Hint: create a failing test file named 'feature-gate-custom_test_frameworks.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(custom_test_frameworks)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-custom_test_frameworks line to the test file.
Expected a gate test for the feature 'cfg_target_abi'.
Hint: create a failing test file named 'feature-gate-cfg_target_abi.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_abi)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_abi line to the test file.
Expected a gate test for the feature 'unsized_locals'.
Hint: create a failing test file named 'feature-gate-unsized_locals.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(unsized_locals)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-unsized_locals line to the test file.
Expected a gate test for the feature 'impl_trait_projections'.
Hint: create a failing test file named 'feature-gate-impl_trait_projections.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(impl_trait_projections)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-impl_trait_projections line to the test file.
Expected a gate test for the feature 'string_deref_patterns'.
Hint: create a failing test file named 'feature-gate-string_deref_patterns.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(string_deref_patterns)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-string_deref_patterns line to the test file.
Expected a gate test for the feature 'ffi_returns_twice'.
Hint: create a failing test file named 'feature-gate-ffi_returns_twice.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ffi_returns_twice)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ffi_returns_twice line to the test file.
Expected a gate test for the feature 'ffi_const'.
Hint: create a failing test file named 'feature-gate-ffi_const.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(ffi_const)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-ffi_const line to the test file.
Expected a gate test for the feature 'movbe_target_feature'.
Hint: create a failing test file named 'feature-gate-movbe_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(movbe_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-movbe_target_feature line to the test file.
Expected a gate test for the feature 'cfg_target_thread_local'.
Hint: create a failing test file named 'feature-gate-cfg_target_thread_local.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(cfg_target_thread_local)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-cfg_target_thread_local line to the test file.
Expected a gate test for the feature 'fn_align'.
Hint: create a failing test file named 'feature-gate-fn_align.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(fn_align)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-fn_align line to the test file.
Expected a gate test for the feature 'sse4a_target_feature'.
Hint: create a failing test file named 'feature-gate-sse4a_target_feature.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(sse4a_target_feature)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-sse4a_target_feature line to the test file.
Expected a gate test for the feature 'wasm_abi'.
Hint: create a failing test file named 'feature-gate-wasm_abi.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(wasm_abi)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-wasm_abi line to the test file.
Expected a gate test for the feature 'const_refs_to_cell'.
Hint: create a failing test file named 'feature-gate-const_refs_to_cell.rs'
