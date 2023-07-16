plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
error: tests/compile-fail/backtrace/bad-backtrace-ptr.rs:7: expected error not found: 0x0 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/backtrace/bad-backtrace-ptr.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/backtrace/bad-backtrace-ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/backtrace/bad-backtrace-ptr.stage-id.aux"
stack backtrace:
unexpected errors (from JSON output): [
    Error {
        line_num: 7,
---
error: tests/compile-fail/dangling_pointers/deref-invalid-ptr.rs:6: expected error not found: inbounds test failed: 0x10 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/dangling_pointers/deref-invalid-ptr.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/dangling_pointers/deref-invalid-ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestMVEI7J/dangling_pointers/deref-invalid-ptr.stage-id.aux"
    Error {
        line_num: 6,
        kind: Some(
            Error,
---
error: tests/compile-fail/dangling_pointers/wild_pointer_deref.rs:3: expected error not found: inbounds test failed: 0x2c is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/dangling_pointers/wild_pointer_deref.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/dangling_pointers/wild_pointer_deref.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/dangling_pointers/wild_pointer_deref.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
error: tests/compile-fail/function_pointers/cast_int_to_fn_ptr.rs:9: expected error not found: inbounds test failed: 0x2a is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/function_pointers/cast_int_to_fn_ptr.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/function_pointers/cast_int_to_fn_ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestMVEI7J/function_pointers/cast_int_to_fn_ptr.stage-id.aux"
    Error {
        line_num: 9,
        kind: Some(
            Error,
---
test [compile-fail] compile-fail/intrinsics/out_of_bounds_ptr_1.rs ... ok

error: error pattern ' inbounds test failed: 0x0 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/ptr_offset_0_plus_0.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/intrinsics/ptr_offset_0_plus_0.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/intrinsics/ptr_offset_0_plus_0.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/intrinsics/ptr_offset_0_plus_0.rs ... FAILED

error: error pattern ' inbounds test failed: 0x1 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/ptr_offset_int_plus_ptr.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/intrinsics/ptr_offset_int_plus_ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/intrinsics/ptr_offset_int_plus_ptr.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/intrinsics/ptr_offset_int_plus_ptr.rs ... FAILED

error: error pattern ' inbounds test failed: 0x1 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/ptr_offset_int_plus_int.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/intrinsics/ptr_offset_int_plus_int.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/intrinsics/ptr_offset_int_plus_int.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
error: tests/compile-fail/null_pointer_deref.rs:3: expected error not found: inbounds test failed: 0x0 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_deref.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/null_pointer_deref.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/null_pointer_deref.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
error: tests/compile-fail/null_pointer_write.rs:3: expected error not found: inbounds test failed: 0x0 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_write.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/null_pointer_write.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/null_pointer_write.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
test [compile-fail] compile-fail/stacked_borrows/load_invalid_mut.rs ... ok

error: error pattern ' inbounds test failed: 0x4 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/issue-miri-1050-2.rs" "-L" "/tmp/compiletestMVEI7J" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMVEI7J/stacked_borrows/issue-miri-1050-2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestMVEI7J/stacked_borrows/issue-miri-1050-2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

---- compile_test stdout ----
diff of stderr:

 error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(clippy::unstable_as_slice)]
    |
    |
    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
 
 error: lint `clippy::unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(clippy::unstable_as_mut_slice)]
 
 
 error: lint `clippy::misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr
    |
    |
 LL | #[warn(clippy::misaligned_transmute)]
 
 
 error: lint `clippy::unused_collect` has been removed: `collect` has been marked as #[must_use] in rustc and that covers all cases of this lint
    |
    |
 LL | #[warn(clippy::unused_collect)]
 
 
 error: lint `clippy::invalid_ref` has been renamed to `invalid_value`
    |
    |
 LL | #[warn(clippy::invalid_ref)]
    |        ^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_value`
 
 error: lint `clippy::into_iter_on_array` has been renamed to `array_into_iter`
    |
    |
 LL | #[warn(clippy::into_iter_on_array)]
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `array_into_iter`
 
 error: lint `clippy::unused_label` has been renamed to `unused_labels`
    |
    |
 LL | #[warn(clippy::unused_label)]
    |        ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unused_labels`
 
 error: lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018
    |
    |
 LL | #[warn(clippy::regex_macro)]
 
 
 error: lint `clippy::drop_bounds` has been renamed to `drop_bounds`
    |
    |
 LL | #[warn(clippy::drop_bounds)]
    |        ^^^^^^^^^^^^^^^^^^^ help: use the new name: `drop_bounds`
 
 error: lint `clippy::temporary_cstring_as_ptr` has been renamed to `temporary_cstring_as_ptr`
    |
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
 LL | #[warn(clippy::temporary_cstring_as_ptr)]
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `temporary_cstring_as_ptr`
 
 error: lint `clippy::panic_params` has been renamed to `non_fmt_panic`
    |
    |
 LL | #[warn(clippy::panic_params)]
    |        ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `non_fmt_panic`
 
 error: lint `clippy::unknown_clippy_lints` has been renamed to `unknown_lints`
    |
    |
 LL | #[warn(clippy::unknown_clippy_lints)]
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unknown_lints`
 
 error: lint `clippy::find_map` has been removed: this lint has been replaced by `manual_find_map`, a more specific lint
    |
    |
 LL | #[warn(clippy::find_map)]
 
 
 error: lint `clippy::filter_map` has been removed: this lint has been replaced by `manual_filter_map`, a more specific lint
    |
    |
 LL | #[warn(clippy::filter_map)]
 
 
-error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
-   |
-   |
-LL | #[warn(clippy::unstable_as_slice)]
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^
-error: aborting due to 15 previous errors
+error: aborting due to 14 previous errors
 
 
---
To only update this specific test, also pass `--test-args deprecated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/deprecated.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/deprecated.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/deprecated.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":7,"byte_end":32,"line_start":1,"line_end":1,"column_start":8,"column_end":33,"is_primary":true,"text":[{"text":"#[warn(clippy::unstable_as_slice)]","highlight_start":8,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D renamed-and-removed-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated.rs:1:8\n   |\nLL | #[warn(clippy::unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D renamed-and-removed-lints` implied by `-D warnings`\n\n"}
{"message":"lint `clippy::unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":42,"byte_end":71,"line_start":2,"line_end":2,"column_start":8,"column_end":37,"is_primary":true,"text":[{"text":"#[warn(clippy::unstable_as_mut_slice)]","highlight_start":8,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated.rs:2:8\n   |\nLL | #[warn(clippy::unstable_as_mut_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":81,"byte_end":109,"line_start":3,"line_end":3,"column_start":8,"column_end":36,"is_primary":true,"text":[{"text":"#[warn(clippy::misaligned_transmute)]","highlight_start":8,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr\n  --> tests/ui/deprecated.rs:3:8\n   |\nLL | #[warn(clippy::misaligned_transmute)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::unused_collect` has been removed: `collect` has been marked as #[must_use] in rustc and that covers all cases of this lint","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":119,"byte_end":141,"line_start":4,"line_end":4,"column_start":8,"column_end":30,"is_primary":true,"text":[{"text":"#[warn(clippy::unused_collect)]","highlight_start":8,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unused_collect` has been removed: `collect` has been marked as #[must_use] in rustc and that covers all cases of this lint\n  --> tests/ui/deprecated.rs:4:8\n   |\nLL | #[warn(clippy::unused_collect)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::invalid_ref` has been renamed to `invalid_value`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":151,"byte_end":170,"line_start":5,"line_end":5,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::invalid_ref)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":151,"byte_end":170,"line_start":5,"line_end":5,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::invalid_ref)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":"invalid_value","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::invalid_ref` has been renamed to `invalid_value`\n  --> tests/ui/deprecated.rs:5:8\n   |\nLL | #[warn(clippy::invalid_ref)]\n   |        ^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_value`\n\n"}
{"message":"lint `clippy::into_iter_on_array` has been renamed to `array_into_iter`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":180,"byte_end":206,"line_start":6,"line_end":6,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"#[warn(clippy::into_iter_on_array)]","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":180,"byte_end":206,"line_start":6,"line_end":6,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"#[warn(clippy::into_iter_on_array)]","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":"array_into_iter","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::into_iter_on_array` has been renamed to `array_into_iter`\n  --> tests/ui/deprecated.rs:6:8\n   |\nLL | #[warn(clippy::into_iter_on_array)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `array_into_iter`\n\n"}
{"message":"lint `clippy::unused_label` has been renamed to `unused_labels`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":216,"byte_end":236,"line_start":7,"line_end":7,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(clippy::unused_label)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":216,"byte_end":236,"line_start":7,"line_end":7,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(clippy::unused_label)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":"unused_labels","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::unused_label` has been renamed to `unused_labels`\n  --> tests/ui/deprecated.rs:7:8\n   |\nLL | #[warn(clippy::unused_label)]\n   |        ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unused_labels`\n\n"}
{"message":"lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":246,"byte_end":265,"line_start":8,"line_end":8,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::regex_macro)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018\n  --> tests/ui/deprecated.rs:8:8\n   |\nLL | #[warn(clippy::regex_macro)]\n   |        ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::drop_bounds` has been renamed to `drop_bounds`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":275,"byte_end":294,"line_start":9,"line_end":9,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::drop_bounds)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":275,"byte_end":294,"line_start":9,"line_end":9,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::drop_bounds)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":"drop_bounds","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::drop_bounds` has been renamed to `drop_bounds`\n  --> tests/ui/deprecated.rs:9:8\n   |\nLL | #[warn(clippy::drop_bounds)]\n   |        ^^^^^^^^^^^^^^^^^^^ help: use the new name: `drop_bounds`\n\n"}
{"message":"lint `clippy::temporary_cstring_as_ptr` has been renamed to `temporary_cstring_as_ptr`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":304,"byte_end":336,"line_start":10,"line_end":10,"column_start":8,"column_end":40,"is_primary":true,"text":[{"text":"#[warn(clippy::temporary_cstring_as_ptr)]","highlight_start":8,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":304,"byte_end":336,"line_start":10,"line_end":10,"column_start":8,"column_end":40,"is_primary":true,"text":[{"text":"#[warn(clippy::temporary_cstring_as_ptr)]","highlight_start":8,"highlight_end":40}],"label":null,"suggested_replacement":"temporary_cstring_as_ptr","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::temporary_cstring_as_ptr` has been renamed to `temporary_cstring_as_ptr`\n  --> tests/ui/deprecated.rs:10:8\n   |\nLL | #[warn(clippy::temporary_cstring_as_ptr)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `temporary_cstring_as_ptr`\n\n"}
{"message":"lint `clippy::panic_params` has been renamed to `non_fmt_panic`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":346,"byte_end":366,"line_start":11,"line_end":11,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(clippy::panic_params)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":346,"byte_end":366,"line_start":11,"line_end":11,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(clippy::panic_params)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":"non_fmt_panic","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::panic_params` has been renamed to `non_fmt_panic`\n  --> tests/ui/deprecated.rs:11:8\n   |\nLL | #[warn(clippy::panic_params)]\n   |        ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `non_fmt_panic`\n\n"}
{"message":"lint `clippy::unknown_clippy_lints` has been renamed to `unknown_lints`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":376,"byte_end":404,"line_start":12,"line_end":12,"column_start":8,"column_end":36,"is_primary":true,"text":[{"text":"#[warn(clippy::unknown_clippy_lints)]","highlight_start":8,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":376,"byte_end":404,"line_start":12,"line_end":12,"column_start":8,"column_end":36,"is_primary":true,"text":[{"text":"#[warn(clippy::unknown_clippy_lints)]","highlight_start":8,"highlight_end":36}],"label":null,"suggested_replacement":"unknown_lints","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::unknown_clippy_lints` has been renamed to `unknown_lints`\n  --> tests/ui/deprecated.rs:12:8\n   |\nLL | #[warn(clippy::unknown_clippy_lints)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unknown_lints`\n\n"}
{"message":"lint `clippy::find_map` has been removed: this lint has been replaced by `manual_find_map`, a more specific lint","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":414,"byte_end":430,"line_start":13,"line_end":13,"column_start":8,"column_end":24,"is_primary":true,"text":[{"text":"#[warn(clippy::find_map)]","highlight_start":8,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::find_map` has been removed: this lint has been replaced by `manual_find_map`, a more specific lint\n  --> tests/ui/deprecated.rs:13:8\n   |\nLL | #[warn(clippy::find_map)]\n   |        ^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::filter_map` has been removed: this lint has been replaced by `manual_filter_map`, a more specific lint","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":440,"byte_end":458,"line_start":14,"line_end":14,"column_start":8,"column_end":26,"is_primary":true,"text":[{"text":"#[warn(clippy::filter_map)]","highlight_start":8,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::filter_map` has been removed: this lint has been replaced by `manual_filter_map`, a more specific lint\n  --> tests/ui/deprecated.rs:14:8\n   |\nLL | #[warn(clippy::filter_map)]\n   |        ^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(unstable_as_slice)]
    |
    |
    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
 
 error: lint `unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(unstable_as_mut_slice)]
 
 
 error: lint `misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr
    |
    |
 LL | #[warn(misaligned_transmute)]
 
 
-error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
-   |
-   |
-LL | #[warn(unstable_as_slice)]
-   |        ^^^^^^^^^^^^^^^^^
-error: aborting due to 4 previous errors
+error: aborting due to 3 previous errors
 
 
---
To only update this specific test, also pass `--test-args deprecated_old.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/deprecated_old.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/deprecated_old.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/deprecated_old.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":7,"byte_end":24,"line_start":1,"line_end":1,"column_start":8,"column_end":25,"is_primary":true,"text":[{"text":"#[warn(unstable_as_slice)]","highlight_start":8,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D renamed-and-removed-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated_old.rs:1:8\n   |\nLL | #[warn(unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D renamed-and-removed-lints` implied by `-D warnings`\n\n"}
{"message":"lint `unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":34,"byte_end":55,"line_start":2,"line_end":2,"column_start":8,"column_end":29,"is_primary":true,"text":[{"text":"#[warn(unstable_as_mut_slice)]","highlight_start":8,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated_old.rs:2:8\n   |\nLL | #[warn(unstable_as_mut_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":65,"byte_end":85,"line_start":3,"line_end":3,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(misaligned_transmute)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr\n  --> tests/ui/deprecated_old.rs:3:8\n   |\nLL | #[warn(misaligned_transmute)]\n   |        ^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`
    |
    |
 LL | #![warn(clippy::cyclomatic_complexity)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::cognitive_complexity`
    |
    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
 
 error: lint `clippy::stutter` has been renamed to `clippy::module_name_repetitions`
    |
    |
 LL | #[warn(clippy::stutter)]
    |        ^^^^^^^^^^^^^^^ help: use the new name: `clippy::module_name_repetitions`
 
 error: lint `clippy::new_without_default_derive` has been renamed to `clippy::new_without_default`
    |
    |
 LL | #[warn(clippy::new_without_default_derive)]
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::new_without_default`
 
 error: lint `clippy::const_static_lifetime` has been renamed to `clippy::redundant_static_lifetimes`
    |
    |
 LL | #[warn(clippy::const_static_lifetime)]
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::redundant_static_lifetimes`
 
-error: lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`
-   |
-   |
-LL | #![warn(clippy::cyclomatic_complexity)]
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::cognitive_complexity`
-error: aborting due to 5 previous errors
+error: aborting due to 4 previous errors
 
 
---
To only update this specific test, also pass `--test-args rename.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/rename.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/rename.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/rename.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":337,"byte_end":366,"line_start":10,"line_end":10,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::cyclomatic_complexity)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D renamed-and-removed-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":337,"byte_end":366,"line_start":10,"line_end":10,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::cyclomatic_complexity)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":"clippy::cognitive_complexity","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`\n  --> tests/ui/rename.rs:10:9\n   |\nLL | #![warn(clippy::cyclomatic_complexity)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::cognitive_complexity`\n   |\n   = note: `-D renamed-and-removed-lints` implied by `-D warnings`\n\n"}
{"message":"lint `clippy::stutter` has been renamed to `clippy::module_name_repetitions`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":377,"byte_end":392,"line_start":12,"line_end":12,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"#[warn(clippy::stutter)]","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":377,"byte_end":392,"line_start":12,"line_end":12,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"#[warn(clippy::stutter)]","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":"clippy::module_name_repetitions","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::stutter` has been renamed to `clippy::module_name_repetitions`\n  --> tests/ui/rename.rs:12:8\n   |\nLL | #[warn(clippy::stutter)]\n   |        ^^^^^^^^^^^^^^^ help: use the new name: `clippy::module_name_repetitions`\n\n"}
{"message":"lint `clippy::new_without_default_derive` has been renamed to `clippy::new_without_default`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":416,"byte_end":450,"line_start":15,"line_end":15,"column_start":8,"column_end":42,"is_primary":true,"text":[{"text":"#[warn(clippy::new_without_default_derive)]","highlight_start":8,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":416,"byte_end":450,"line_start":15,"line_end":15,"column_start":8,"column_end":42,"is_primary":true,"text":[{"text":"#[warn(clippy::new_without_default_derive)]","highlight_start":8,"highlight_end":42}],"label":null,"suggested_replacement":"clippy::new_without_default","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::new_without_default_derive` has been renamed to `clippy::new_without_default`\n  --> tests/ui/rename.rs:15:8\n   |\nLL | #[warn(clippy::new_without_default_derive)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::new_without_default`\n\n"}
{"message":"lint `clippy::const_static_lifetime` has been renamed to `clippy::redundant_static_lifetimes`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":473,"byte_end":502,"line_start":18,"line_end":18,"column_start":8,"column_end":37,"is_primary":true,"text":[{"text":"#[warn(clippy::const_static_lifetime)]","highlight_start":8,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":473,"byte_end":502,"line_start":18,"line_end":18,"column_start":8,"column_end":37,"is_primary":true,"text":[{"text":"#[warn(clippy::const_static_lifetime)]","highlight_start":8,"highlight_end":37}],"label":null,"suggested_replacement":"clippy::redundant_static_lifetimes","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::const_static_lifetime` has been renamed to `clippy::redundant_static_lifetimes`\n  --> tests/ui/rename.rs:18:8\n   |\nLL | #[warn(clippy::const_static_lifetime)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::redundant_static_lifetimes`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
