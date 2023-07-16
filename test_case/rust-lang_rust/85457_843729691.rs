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
failures:

---- compile_test stdout ----
normalized stderr:
error: the feature `extended_key_value_attributes` has been stable since 1.54.0 and no longer requires an attribute to enable
   |
LL | #![feature(extended_key_value_attributes)]
error: test failed, to rerun pass '--test compile-test'
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args missing-doc-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/missing-doc-crate.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/missing-doc-crate.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/missing-doc-crate.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `extended_key_value_attributes` has been stable since 1.54.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc-crate.rs","byte_start":59,"byte_end":88,"line_start":2,"line_end":2,"column_start":12,"column_end":41,"is_primary":true,"text":[{"text":"#![feature(extended_key_value_attributes)]","highlight_start":12,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `extended_key_value_attributes` has been stable since 1.54.0 and no longer requires an attribute to enable\n  --> tests/ui/missing-doc-crate.rs:2:12\n   |\nLL | #![feature(extended_key_value_attributes)]\n   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
