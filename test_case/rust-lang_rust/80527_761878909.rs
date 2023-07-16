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
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
Executing the job since clippy subtree was updated
with:
  github_token: ***
  check_every_seconds: 60
env:
---
failures:

---- compile_test stdout ----
normalized stderr:
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


error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
   |
   |
LL | #[warn(unstable_as_slice)]

error: aborting due to 4 previous errors




expected stderr:
error: lint `unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
   |
   |
LL | #[warn(unstable_as_slice)]
   |
   |
   = note: `-D renamed-and-removed-lints` implied by `-D warnings`

error: lint `unstable_as_mut_slice` has been removed: ``Vec::as_mut_slice` has been stabilized in 1.7`
   |
   |
LL | #[warn(unstable_as_mut_slice)]


error: lint `misaligned_transmute` has been removed: `this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr`
   |
   |
LL | #[warn(misaligned_transmute)]


error: lint `unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
   |
   |
LL | #[warn(unstable_as_slice)]

error: aborting due to 4 previous errors




diff of stderr:

error: test failed, to rerun pass '--test compile-test'
-error: lint `unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
+error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(unstable_as_slice)]
    |
    |
    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
 
-error: lint `unstable_as_mut_slice` has been removed: ``Vec::as_mut_slice` has been stabilized in 1.7`
+error: lint `unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(unstable_as_mut_slice)]
 
 
-error: lint `misaligned_transmute` has been removed: `this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr`
+error: lint `misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr
    |
    |
 LL | #[warn(misaligned_transmute)]
 
 
-error: lint `unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
+error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(unstable_as_slice)]
 
 error: aborting due to 4 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/deprecated_old.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base' 'deprecated_old.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/deprecated_old.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/deprecated_old.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-bd8c817a160fb1e1.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-e3c044d770c3edb5.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/deprecated_old.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":7,"byte_end":24,"line_start":1,"line_end":1,"column_start":8,"column_end":25,"is_primary":true,"text":[{"text":"#[warn(unstable_as_slice)]","highlight_start":8,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D renamed-and-removed-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated_old.rs:1:8\n   |\nLL | #[warn(unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D renamed-and-removed-lints` implied by `-D warnings`\n\n"}
{"message":"lint `unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":34,"byte_end":55,"line_start":2,"line_end":2,"column_start":8,"column_end":29,"is_primary":true,"text":[{"text":"#[warn(unstable_as_mut_slice)]","highlight_start":8,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated_old.rs:2:8\n   |\nLL | #[warn(unstable_as_mut_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":65,"byte_end":85,"line_start":3,"line_end":3,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(misaligned_transmute)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr\n  --> tests/ui/deprecated_old.rs:3:8\n   |\nLL | #[warn(misaligned_transmute)]\n   |        ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated_old.rs","byte_start":7,"byte_end":24,"line_start":1,"line_end":1,"column_start":8,"column_end":25,"is_primary":true,"text":[{"text":"#[warn(unstable_as_slice)]","highlight_start":8,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated_old.rs:1:8\n   |\nLL | #[warn(unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
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


error: lint `clippy::invalid_ref` has been removed: superseded by rustc lint `invalid_value`
   |
   |
LL | #[warn(clippy::invalid_ref)]


error: lint `clippy::into_iter_on_array` has been removed: this lint has been uplifted to rustc and is now called `array_into_iter`
   |
   |
LL | #[warn(clippy::into_iter_on_array)]


error: lint `clippy::unused_label` has been removed: this lint has been uplifted to rustc and is now called `unused_labels`
   |
   |
LL | #[warn(clippy::unused_label)]


error: lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018
   |
   |
LL | #[warn(clippy::regex_macro)]


error: lint `clippy::drop_bounds` has been removed: this lint has been uplifted to rustc and is now called `drop_bounds`
   |
   |
LL | #[warn(clippy::drop_bounds)]


error: lint `clippy::temporary_cstring_as_ptr` has been removed: this lint has been uplifted to rustc and is now called `temporary_cstring_as_ptr`
   |
   |
LL | #[warn(clippy::temporary_cstring_as_ptr)]


error: lint `clippy::panic_params` has been removed: this lint has been uplifted to rustc and is now called `panic_fmt`
   |
   |
LL | #[warn(clippy::panic_params)]


error: lint `clippy::unknown_clippy_lints` has been removed: this lint has been integrated into the `unknown_lints` rustc lint
   |
   |
LL | #[warn(clippy::unknown_clippy_lints)]


error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
   |
   |
LL | #[warn(clippy::unstable_as_slice)]

error: aborting due to 13 previous errors




expected stderr:
error: lint `clippy::unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
   |
   |
LL | #[warn(clippy::unstable_as_slice)]
   |
   |
   = note: `-D renamed-and-removed-lints` implied by `-D warnings`

error: lint `clippy::unstable_as_mut_slice` has been removed: ``Vec::as_mut_slice` has been stabilized in 1.7`
   |
   |
LL | #[warn(clippy::unstable_as_mut_slice)]


error: lint `clippy::misaligned_transmute` has been removed: `this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr`
   |
   |
LL | #[warn(clippy::misaligned_transmute)]


error: lint `clippy::unused_collect` has been removed: ``collect` has been marked as #[must_use] in rustc and that covers all cases of this lint`
   |
   |
LL | #[warn(clippy::unused_collect)]


error: lint `clippy::invalid_ref` has been removed: `superseded by rustc lint `invalid_value``
   |
   |
LL | #[warn(clippy::invalid_ref)]


error: lint `clippy::into_iter_on_array` has been removed: `this lint has been uplifted to rustc and is now called `array_into_iter``
   |
   |
LL | #[warn(clippy::into_iter_on_array)]


error: lint `clippy::unused_label` has been removed: `this lint has been uplifted to rustc and is now called `unused_labels``
   |
   |
LL | #[warn(clippy::unused_label)]


error: lint `clippy::regex_macro` has been removed: `the regex! macro has been removed from the regex crate in 2018`
   |
   |
LL | #[warn(clippy::regex_macro)]


error: lint `clippy::drop_bounds` has been removed: `this lint has been uplifted to rustc and is now called `drop_bounds``
   |
   |
LL | #[warn(clippy::drop_bounds)]


error: lint `clippy::temporary_cstring_as_ptr` has been removed: `this lint has been uplifted to rustc and is now called `temporary_cstring_as_ptr``
   |
   |
LL | #[warn(clippy::temporary_cstring_as_ptr)]


error: lint `clippy::panic_params` has been removed: `this lint has been uplifted to rustc and is now called `panic_fmt``
   |
   |
LL | #[warn(clippy::panic_params)]


error: lint `clippy::unknown_clippy_lints` has been removed: `this lint has been integrated into the `unknown_lints` rustc lint`
   |
   |
LL | #[warn(clippy::unknown_clippy_lints)]


error: lint `clippy::unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
   |
   |
LL | #[warn(clippy::unstable_as_slice)]

error: aborting due to 13 previous errors




diff of stderr:

-error: lint `clippy::unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
+error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(clippy::unstable_as_slice)]
    |
    |
    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
 
-error: lint `clippy::unstable_as_mut_slice` has been removed: ``Vec::as_mut_slice` has been stabilized in 1.7`
+error: lint `clippy::unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(clippy::unstable_as_mut_slice)]
 
 
-error: lint `clippy::misaligned_transmute` has been removed: `this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr`
+error: lint `clippy::misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr
    |
    |
 LL | #[warn(clippy::misaligned_transmute)]
 
 
-error: lint `clippy::unused_collect` has been removed: ``collect` has been marked as #[must_use] in rustc and that covers all cases of this lint`
+error: lint `clippy::unused_collect` has been removed: `collect` has been marked as #[must_use] in rustc and that covers all cases of this lint
    |
    |
 LL | #[warn(clippy::unused_collect)]
 
 
-error: lint `clippy::invalid_ref` has been removed: `superseded by rustc lint `invalid_value``
+error: lint `clippy::invalid_ref` has been removed: superseded by rustc lint `invalid_value`
    |
    |
 LL | #[warn(clippy::invalid_ref)]
 
 
-error: lint `clippy::into_iter_on_array` has been removed: `this lint has been uplifted to rustc and is now called `array_into_iter``
+error: lint `clippy::into_iter_on_array` has been removed: this lint has been uplifted to rustc and is now called `array_into_iter`
    |
    |
 LL | #[warn(clippy::into_iter_on_array)]
 
 
-error: lint `clippy::unused_label` has been removed: `this lint has been uplifted to rustc and is now called `unused_labels``
+error: lint `clippy::unused_label` has been removed: this lint has been uplifted to rustc and is now called `unused_labels`
    |
    |
 LL | #[warn(clippy::unused_label)]
 
 
-error: lint `clippy::regex_macro` has been removed: `the regex! macro has been removed from the regex crate in 2018`
+error: lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018
    |
    |
 LL | #[warn(clippy::regex_macro)]
 
 
-error: lint `clippy::drop_bounds` has been removed: `this lint has been uplifted to rustc and is now called `drop_bounds``
+error: lint `clippy::drop_bounds` has been removed: this lint has been uplifted to rustc and is now called `drop_bounds`
    |
    |
 LL | #[warn(clippy::drop_bounds)]
 
 
-error: lint `clippy::temporary_cstring_as_ptr` has been removed: `this lint has been uplifted to rustc and is now called `temporary_cstring_as_ptr``
+error: lint `clippy::temporary_cstring_as_ptr` has been removed: this lint has been uplifted to rustc and is now called `temporary_cstring_as_ptr`
    |
    |
 LL | #[warn(clippy::temporary_cstring_as_ptr)]
 
 
-error: lint `clippy::panic_params` has been removed: `this lint has been uplifted to rustc and is now called `panic_fmt``
+error: lint `clippy::panic_params` has been removed: this lint has been uplifted to rustc and is now called `panic_fmt`
    |
    |
 LL | #[warn(clippy::panic_params)]
 
 
-error: lint `clippy::unknown_clippy_lints` has been removed: `this lint has been integrated into the `unknown_lints` rustc lint`
+error: lint `clippy::unknown_clippy_lints` has been removed: this lint has been integrated into the `unknown_lints` rustc lint
    |
    |
 LL | #[warn(clippy::unknown_clippy_lints)]
 
 
-error: lint `clippy::unstable_as_slice` has been removed: ``Vec::as_slice` has been stabilized in 1.7`
+error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7
    |
    |
 LL | #[warn(clippy::unstable_as_slice)]
 
 error: aborting due to 13 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/deprecated.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base' 'deprecated.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/deprecated.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/deprecated.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-bd8c817a160fb1e1.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-e3c044d770c3edb5.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/deprecated.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":7,"byte_end":32,"line_start":1,"line_end":1,"column_start":8,"column_end":33,"is_primary":true,"text":[{"text":"#[warn(clippy::unstable_as_slice)]","highlight_start":8,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D renamed-and-removed-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated.rs:1:8\n   |\nLL | #[warn(clippy::unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D renamed-and-removed-lints` implied by `-D warnings`\n\n"}
{"message":"lint `clippy::unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":42,"byte_end":71,"line_start":2,"line_end":2,"column_start":8,"column_end":37,"is_primary":true,"text":[{"text":"#[warn(clippy::unstable_as_mut_slice)]","highlight_start":8,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unstable_as_mut_slice` has been removed: `Vec::as_mut_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated.rs:2:8\n   |\nLL | #[warn(clippy::unstable_as_mut_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":81,"byte_end":109,"line_start":3,"line_end":3,"column_start":8,"column_end":36,"is_primary":true,"text":[{"text":"#[warn(clippy::misaligned_transmute)]","highlight_start":8,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::misaligned_transmute` has been removed: this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr\n  --> tests/ui/deprecated.rs:3:8\n   |\nLL | #[warn(clippy::misaligned_transmute)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::unused_collect` has been removed: `collect` has been marked as #[must_use] in rustc and that covers all cases of this lint","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":119,"byte_end":141,"line_start":4,"line_end":4,"column_start":8,"column_end":30,"is_primary":true,"text":[{"text":"#[warn(clippy::unused_collect)]","highlight_start":8,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unused_collect` has been removed: `collect` has been marked as #[must_use] in rustc and that covers all cases of this lint\n  --> tests/ui/deprecated.rs:4:8\n   |\nLL | #[warn(clippy::unused_collect)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::invalid_ref` has been removed: superseded by rustc lint `invalid_value`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":151,"byte_end":170,"line_start":5,"line_end":5,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::invalid_ref)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::invalid_ref` has been removed: superseded by rustc lint `invalid_value`\n  --> tests/ui/deprecated.rs:5:8\n   |\nLL | #[warn(clippy::invalid_ref)]\n   |        ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::into_iter_on_array` has been removed: this lint has been uplifted to rustc and is now called `array_into_iter`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":180,"byte_end":206,"line_start":6,"line_end":6,"column_start":8,"column_end":34,"is_primary":true,"text":[{"text":"#[warn(clippy::into_iter_on_array)]","highlight_start":8,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::into_iter_on_array` has been removed: this lint has been uplifted to rustc and is now called `array_into_iter`\n  --> tests/ui/deprecated.rs:6:8\n   |\nLL | #[warn(clippy::into_iter_on_array)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::unused_label` has been removed: this lint has been uplifted to rustc and is now called `unused_labels`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":216,"byte_end":236,"line_start":7,"line_end":7,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(clippy::unused_label)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unused_label` has been removed: this lint has been uplifted to rustc and is now called `unused_labels`\n  --> tests/ui/deprecated.rs:7:8\n   |\nLL | #[warn(clippy::unused_label)]\n   |        ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":246,"byte_end":265,"line_start":8,"line_end":8,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::regex_macro)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::regex_macro` has been removed: the regex! macro has been removed from the regex crate in 2018\n  --> tests/ui/deprecated.rs:8:8\n   |\nLL | #[warn(clippy::regex_macro)]\n   |        ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::drop_bounds` has been removed: this lint has been uplifted to rustc and is now called `drop_bounds`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":275,"byte_end":294,"line_start":9,"line_end":9,"column_start":8,"column_end":27,"is_primary":true,"text":[{"text":"#[warn(clippy::drop_bounds)]","highlight_start":8,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::drop_bounds` has been removed: this lint has been uplifted to rustc and is now called `drop_bounds`\n  --> tests/ui/deprecated.rs:9:8\n   |\nLL | #[warn(clippy::drop_bounds)]\n   |        ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::temporary_cstring_as_ptr` has been removed: this lint has been uplifted to rustc and is now called `temporary_cstring_as_ptr`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":304,"byte_end":336,"line_start":10,"line_end":10,"column_start":8,"column_end":40,"is_primary":true,"text":[{"text":"#[warn(clippy::temporary_cstring_as_ptr)]","highlight_start":8,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::temporary_cstring_as_ptr` has been removed: this lint has been uplifted to rustc and is now called `temporary_cstring_as_ptr`\n  --> tests/ui/deprecated.rs:10:8\n   |\nLL | #[warn(clippy::temporary_cstring_as_ptr)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::panic_params` has been removed: this lint has been uplifted to rustc and is now called `panic_fmt`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":346,"byte_end":366,"line_start":11,"line_end":11,"column_start":8,"column_end":28,"is_primary":true,"text":[{"text":"#[warn(clippy::panic_params)]","highlight_start":8,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::panic_params` has been removed: this lint has been uplifted to rustc and is now called `panic_fmt`\n  --> tests/ui/deprecated.rs:11:8\n   |\nLL | #[warn(clippy::panic_params)]\n   |        ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::unknown_clippy_lints` has been removed: this lint has been integrated into the `unknown_lints` rustc lint","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":376,"byte_end":404,"line_start":12,"line_end":12,"column_start":8,"column_end":36,"is_primary":true,"text":[{"text":"#[warn(clippy::unknown_clippy_lints)]","highlight_start":8,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unknown_clippy_lints` has been removed: this lint has been integrated into the `unknown_lints` rustc lint\n  --> tests/ui/deprecated.rs:12:8\n   |\nLL | #[warn(clippy::unknown_clippy_lints)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/deprecated.rs","byte_start":7,"byte_end":32,"line_start":1,"line_end":1,"column_start":8,"column_end":33,"is_primary":true,"text":[{"text":"#[warn(clippy::unstable_as_slice)]","highlight_start":8,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lint `clippy::unstable_as_slice` has been removed: `Vec::as_slice` has been stabilized in 1.7\n  --> tests/ui/deprecated.rs:1:8\n   |\nLL | #[warn(clippy::unstable_as_slice)]\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22
