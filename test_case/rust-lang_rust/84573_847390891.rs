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
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 126561cb31e8ebe1e2dd9dfd0d3ca621308dc56f and 77ac15d70c099115ea1a8ee954484e97bbce5dff
Executing the job since clippy or rustfmt subtree was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
---- compile_test stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/infallible_destructuring_match.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-cbc4a79cec74ade8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/infallible_destructuring_match.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.fixed","byte_start":47,"byte_end":57,"line_start":2,"line_end":2,"column_start":33,"column_end":43,"is_primary":true,"text":[{"text":"#![feature(exhaustive_patterns, never_type)]","highlight_start":33,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `never_type` has been stable since 1.54.0 and no longer requires an attribute to enable\n  --> tests/ui/infallible_destructuring_match.fixed:2:33\n   |\nLL | #![feature(exhaustive_patterns, never_type)]\n   |                                 ^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}

------------------------------------------

error: test failed, to rerun pass '--test compile-test'
