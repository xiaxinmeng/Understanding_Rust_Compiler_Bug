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
    |        ^^^^^^^^^^^^^^^^^^^^^^
 
 
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
    |
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
 
-error: lint `clippy::invalid_atomic_ordering` has been renamed to `invalid_atomic_ordering`
-   |
-   |
-LL | #[warn(clippy::invalid_atomic_ordering)]
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_atomic_ordering`
-
 error: lint `clippy::find_map` has been removed: this lint has been replaced by `manual_find_map`, a more specific lint
    |
    |
 LL | #[warn(clippy::find_map)]
 
 
 error: lint `clippy::filter_map` has been removed: this lint has been replaced by `manual_filter_map`, a more specific lint
    |
    |
 LL | #[warn(clippy::filter_map)]
 
 
+error: lint `clippy::invalid_atomic_ordering` has been removed: invalid_atomic_ordering
+   |
+   |
+LL | #[warn(clippy::invalid_atomic_ordering)]
+
+
 error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(clippy::unstable_as_slice)]
 
-error: aborting due to 15 previous errors
+error: aborting due to 16 previous errors
 
---
To only update this specific test, also pass `--test-args deprecated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/deprecated.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/deprecated.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/deprecated.stage-id.aux"
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
{"message":"lint `clippy::invalid_atomic_ordering` has been removed: invalid_atomic_ordering","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":468,"byte_end":499,"line_start":15,"line_end":15,"column_start":8,"column_end":39,"is_primary":true,"text":[{"text":"#[warn(clippy::invalid_atomic_ordering)]","highlight_start":8,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::invalid_atomic_ordering` has been removed: invalid_atomic_ordering\n  --> tests/ui/deprecated.rs:15:8\n   |\nLL | #[warn(clippy::invalid_atomic_ordering)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":7,"byte_end":32,"line_start":1,"line_end":1,"column_start":8,"column_end":33,"is_primary":true,"text":[{"text":"#[warn(clippy::unstable_as_slice)]","highlight_start":8,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated.rs:1:8\n   |\nLL | #[warn(clippy::unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
