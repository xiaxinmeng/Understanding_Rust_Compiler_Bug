plain
[00:01:03] Successfully tagged rust-ci:latest
[00:01:03] Built container sha256:37165909d78207f4229a97c5cd0fabd9eefd753d100a47f0ce0efe526d0a9eab
[00:01:03] Uploading finished image to s3://rust-lang-ci-sccache2/docker/3cda91c4db4241a6d70dd09f39ec5d1d9a7d6811541a18ccabfdca3ec1fe0d88aa83411a8f416d9d321e909bff6742ed11fb126da6b2692a997c656dd97ac60c
[00:01:03] 
[00:01:03] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:01:07] xargs: docker: terminated by signal 13

[00:01:07] travis_time:end:2bcc60f9:start=1532773227587391459,finish=1532773283196487791,duration=55609096332
[CI_JOB_NAME=x86_64-gnu-tools]
[00:01:07] [CI_JOB_NAME=x86_64-gnu-tools]
---
[00:51:48] Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:51:48] travis_fold:start:stage2-clippy-driver
travis_time:start:stage2-clippy-driver
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path: list_files_git clippy v0.0.212 (file:///checkout/src/tools/clippy)
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.editorconfig
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.github/ISSUE_TEMPLATE.md
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.github/deploy.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.github/deploy_key.enc
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.gitignore
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.remarkrc.json
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/.travis.yml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/CHANGELOG.md
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/CONTRIBUTING.md
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/Cargo.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/LICENSE
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/PUBLISH.md
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/README.md
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/appveyor.yml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/build.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/ci/base-tests.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/ci/integration-tests.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path: subpackage found: /checkout/src/tools/clippy/clippy_lints
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path: subpackage found: /checkout/src/tools/clippy/clippy_workspace_tests
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path: subpackage found: /checkout/src/tools/clippy/clippy_workspace_tests/subcrate
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/min_version.txt
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path: subpackage found: /checkout/src/tools/clippy/mini-macro
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/pre_publish.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/publish.files
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/rust-toolchain
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/rust-update
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/rustfmt.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/src/driver.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/src/lib.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/src/main.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/auxiliary/test_macro.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/camel_case.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/compile-test.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/dogfood.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/matches.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/needless_continue_helpers.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/associated-constant-ice.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/cc_seme.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/enum-glob-import-crate.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-1588.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-1782.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-1969.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-2499.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-2594.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-2727.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-2760.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-2774.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-2865.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice-700.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/ice_exacte_size.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/if_same_then_else.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/issue-2862.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/issue-825.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/issues_loop_mut_cond.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/match_same_arms_const.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/mut_mut_macro.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/needless_borrow_fp.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/needless_lifetimes_impl_trait.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/procedural_macro.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/regressions.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/returns.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/single-match-else.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/used_underscore_binding_macro.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/whitelist/clippy.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/run-pass/whitelist/conf_whitelisted.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/trim_multiline.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/bad_toml/clippy.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/bad_toml/conf_bad_toml.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/bad_toml/conf_bad_toml.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/bad_toml_type/clippy.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/bad_toml_type/conf_bad_type.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/bad_toml_type/conf_bad_type.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_blacklist/clippy.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_blacklist/conf_french_blacklisted_name.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_blacklist/conf_french_blacklisted_name.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_trivially_copy/clippy.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_trivially_copy/test.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_trivially_copy/test.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_unknown_key/clippy.toml
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_unknown_key/conf_unknown_key.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/toml_unknown_key/conf_unknown_key.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/update-all-references.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui-toml/update-references.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/absurd-extreme-comparisons.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/absurd-extreme-comparisons.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/approx_const.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/approx_const.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/arithmetic.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/arithmetic.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/assign_ops.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/assign_ops.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/assign_ops2.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/assign_ops2.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/attrs.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/attrs.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/call.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/call.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/for_loop.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/for_loop.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/matches.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/matches.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/author/matches.stout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/bit_masks.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/bit_masks.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/blacklisted_name.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/blacklisted_name.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/block_in_if_condition.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/block_in_if_condition.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/bool_comparison.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/bool_comparison.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/booleans.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/booleans.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/borrow_box.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/borrow_box.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/box_vec.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/box_vec.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/builtin-type-shadow.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/builtin-type-shadow.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/bytecount.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/bytecount.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_alignment.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_alignment.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_lossless_float.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_lossless_float.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_lossless_integer.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_lossless_integer.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_size.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cast_size.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/char_lit_as_u8.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/char_lit_as_u8.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/checked_unwrap.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/checked_unwrap.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/clone_on_copy_impl.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/clone_on_copy_mut.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cmp_nan.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cmp_nan.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cmp_null.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cmp_null.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cmp_owned.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cmp_owned.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/collapsible_if.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/collapsible_if.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/complex_types.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/complex_types.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/const_static_lifetime.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/const_static_lifetime.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/copies.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/copies.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cstring.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cstring.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cyclomatic_complexity.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cyclomatic_complexity.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cyclomatic_complexity_attr_used.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/cyclomatic_complexity_attr_used.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/decimal_literal_representation.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/decimal_literal_representation.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/default_trait_access.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/default_trait_access.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/default_trait_access.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/deprecated.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/deprecated.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/derive.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/derive.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/diverging_sub_expression.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/diverging_sub_expression.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/dlist.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/dlist.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/doc.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/doc.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/double_comparison.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/double_comparison.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/double_neg.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/double_neg.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/double_parens.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/double_parens.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/drop_forget_copy.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/drop_forget_copy.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/drop_forget_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/drop_forget_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/duplicate_underscore_argument.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/duplicate_underscore_argument.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/duration_subsec.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/duration_subsec.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/duration_subsec.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/else_if_without_else.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/else_if_without_else.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/empty_enum.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/empty_enum.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/empty_line_after_outer_attribute.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/empty_line_after_outer_attribute.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/entry.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/entry.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/enum_glob_use.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/enum_glob_use.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/enum_variants.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/enum_variants.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/enums_clike.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/enums_clike.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/eq_op.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/eq_op.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/erasing_op.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/erasing_op.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/escape_analysis.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/escape_analysis.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/eta.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/eta.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/eval_order_dependence.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/eval_order_dependence.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/excessive_precision.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/excessive_precision.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/explicit_write.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/explicit_write.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/fallible_impl_from.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/fallible_impl_from.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/filter_methods.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/filter_methods.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/float_cmp.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/float_cmp.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/float_cmp_const.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/float_cmp_const.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/for_loop.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/for_loop.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/format.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/format.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/formatting.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/formatting.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/functions.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/functions.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/get_unwrap.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/get_unwrap.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/identity_conversion.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/identity_conversion.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/identity_op.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/identity_op.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/if_let_redundant_pattern_matching.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/if_let_redundant_pattern_matching.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/if_not_else.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/if_not_else.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/impl.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/impl.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/implicit_hasher.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/implicit_hasher.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/inconsistent_digit_grouping.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/inconsistent_digit_grouping.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/indexing_slicing.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/indexing_slicing.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/infallible_destructuring_match.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/infallible_destructuring_match.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/infinite_iter.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/infinite_iter.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/infinite_loop.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/infinite_loop.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/inline_fn_without_body.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/inline_fn_without_body.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/int_plus_one.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/int_plus_one.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/invalid_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/invalid_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/invalid_upcast_comparisons.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/invalid_upcast_comparisons.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/issue_2356.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/issue_2356.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/item_after_statement.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/item_after_statement.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/large_digit_groups.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/large_digit_groups.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/large_enum_variant.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/large_enum_variant.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/len_zero.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/len_zero.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/let_if_seq.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/let_if_seq.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/let_return.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/let_return.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/let_unit.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/let_unit.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/lifetimes.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/lifetimes.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/literals.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/literals.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/map_clone.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/map_clone.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/map_unit_fn.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/match_bool.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/match_bool.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/matches.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/matches.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mem_forget.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mem_forget.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/methods.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/methods.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/min_max.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/min_max.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/missing-doc.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/missing-doc.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/missing_inline.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/missing_inline.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/module_inception.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/module_inception.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/modulo_one.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/modulo_one.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_from_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_from_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_mut.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_mut.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_range_bound.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_range_bound.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_reference.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mut_reference.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mutex_atomic.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/mutex_atomic.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_bool.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_bool.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_borrow.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_borrow.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_borrowed_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_borrowed_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_continue.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_continue.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_pass_by_value.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_pass_by_value.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_pass_by_value_proc_macro.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_range_loop.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_range_loop.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_return.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_return.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_update.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/needless_update.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/neg_cmp_op_on_partial_ord.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/neg_cmp_op_on_partial_ord.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/neg_multiply.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/neg_multiply.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/never_loop.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/never_loop.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/new_without_default.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/new_without_default.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/no_effect.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/no_effect.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/non_copy_const.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/non_copy_const.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/non_expressive_names.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/non_expressive_names.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ok_expect.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ok_expect.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ok_if_let.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ok_if_let.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/op_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/op_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/open_options.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/open_options.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/option_map_unit_fn.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/option_map_unit_fn.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/option_option.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/option_option.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/overflow_check_conditional.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/overflow_check_conditional.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/panic_unimplemented.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/panic_unimplemented.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/partialeq_ne_impl.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/partialeq_ne_impl.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/patterns.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/patterns.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/precedence.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/precedence.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/print.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/print.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/print_literal.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/print_literal.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/print_with_newline.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/print_with_newline.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/println_empty_string.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/println_empty_string.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ptr_arg.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ptr_arg.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/question_mark.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/question_mark.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/range.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/range.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/range_plus_minus_one.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/range_plus_minus_one.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/redundant_closure_call.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/redundant_closure_call.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/redundant_field_names.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/redundant_field_names.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/reference.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/reference.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/regex.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/regex.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/replace_consts.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/replace_consts.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/result_map_unit_fn.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/result_map_unit_fn.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/serde.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/serde.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/shadow.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/shadow.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/short_circuit_statement.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/short_circuit_statement.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/single_char_pattern.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/single_char_pattern.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/single_match.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/single_match.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/starts_ends_with.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/starts_ends_with.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/string_extend.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/string_extend.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/strings.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/strings.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/stutter.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/stutter.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/suspicious_arithmetic_impl.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/suspicious_arithmetic_impl.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/swap.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/swap.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/temporary_assignment.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/temporary_assignment.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/toplevel_ref_arg.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/toplevel_ref_arg.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/trailing_zeros.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/trailing_zeros.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/trailing_zeros.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/transmute.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/transmute.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/transmute_32bit.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/transmute_64bit.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/transmute_64bit.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/trivially_copy_pass_by_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/trivially_copy_pass_by_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/ty_fn_sig.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/types.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/types.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/types_fn_to_int.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/types_fn_to_int.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unicode.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unicode.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unit_arg.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unit_arg.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unit_cmp.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unit_cmp.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unnecessary_clone.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unnecessary_clone.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unnecessary_fold.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unnecessary_fold.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unnecessary_ref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unnecessary_ref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unneeded_field_pattern.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unneeded_field_pattern.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unreadable_literal.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unreadable_literal.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unsafe_removed_from_name.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unsafe_removed_from_name.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unused_io_amount.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unused_io_amount.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unused_labels.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unused_labels.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unused_lt.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unused_lt.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unwrap_or.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unwrap_or.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/unwrap_or.stdout
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/update-all-references.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/update-references.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/use_self.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/use_self.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/used_underscore_binding.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/used_underscore_binding.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/useless_asref.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/useless_asref.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/useless_attribute.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/useless_attribute.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/vec.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/vec.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/while_loop.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/while_loop.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/write_literal.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/write_literal.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/write_with_newline.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/write_with_newline.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/writeln_empty_string.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/writeln_empty_string.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/wrong_self_convention.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/wrong_self_convention.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/zero_div_zero.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/zero_div_zero.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/zero_ptr.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/ui/zero_ptr.stderr
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/versioncheck.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/tests/without_block_comments.rs
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/util/cov.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/util/dogfood.sh
[00:51:48]  WARN 2018-07-28T11:12:04Z: cargo::sources::path:   found /checkout/src/tools/clippy/util/export.py
---
[01:01:40] Installing openssl for x86_64-unknown-linux-gnu
[01:02:09] travis_fold:start:stage2-rls
travis_time:start:stage2-rls
Building stage2 tool rls (x86_64-unknown-linux-gnu)
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: list_files_git rls v0.129.0 (file:///checkout/src/tools/rls)
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/.gitignore
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/.travis.yml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/COPYRIGHT
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/Cargo.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/LICENSE-APACHE
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/LICENSE-MIT
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/README.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/appveyor.yml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/bors.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/build.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/clients.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/contributing.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/debugging.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/diagnostics.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/notifications.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/post_build.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/progress.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/requests.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/run.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/actions/work_pool.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/build/cargo.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/build/environment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/build/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/build/plan.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/build/rustc.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/cmd.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/concurrency.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/config.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/lsp_data.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/main.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/server/dispatch.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/server/io.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/server/message.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/server/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/test/harness.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/test/lens.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/src/test/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/bin_lib
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/borrow_error
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/common
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/compile_fail
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/deglob
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/dep_fail
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/features
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/find_all_refs_no_cfg_test
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/find_impls
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/infer_bin
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/infer_custom_bin
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/infer_lib
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/lens_run
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/multiple_bins
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/reformat
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/reformat_with_range
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/workspace_symbol
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: subpackage found: /checkout/src/tools/rls/test_data/workspace_symbol_duplicates
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/tests/support/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/tests/support/paths.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rls/tests/tests.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path: list_files_git rustfmt-nightly v0.9.0 (file:///checkout/src/tools/rustfmt)
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/.gitignore
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/.travis.yml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/CHANGELOG.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/CODE_OF_CONDUCT.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/Cargo.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/Configurations.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/Contributing.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/Design.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/LICENSE-APACHE
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/LICENSE-MIT
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/README.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/appveyor.yml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/atom.md
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/bootstrap.sh
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/build.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/ci/integration.sh
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/legacy-rustfmt.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/rustfmt.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/attr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/bin/main.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/cargo-fmt/main.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/chains.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/checkstyle.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/closures.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/codemap.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/comment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/config/config_type.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/config/file_lines.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/config/license.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/config/lists.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/config/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/config/options.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/expr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/filemap.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/format-diff/main.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/format-diff/test/bindgen.diff
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/formatting.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/git-rustfmt/main.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/imports.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/issues.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/items.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/lib.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/lists.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/macros.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/matches.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/missed_spans.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/modules.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/overflow.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/pairs.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/patterns.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/reorder.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/rewrite.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/rustfmt_diff.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/shape.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/spanned.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/string.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/test/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/types.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/utils.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/vertical.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/src/visitor.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/config/disable_all_formatting.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/config/issue-1111.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/config/issue-2641.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/config/skip_children.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/config/small_tabs.toml
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/coverage/source/comments.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/coverage/target/comments.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/array_comment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/assignment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/associated-types-bounds-wrapping.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/attrib.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/big-impl-block.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/big-impl-visual.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/binary-expr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/break-and-continue.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/catch.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/chains-visual.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/chains.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/closure-block-inside-macro.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/closure.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/comment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/comment2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/comment3.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/comment4.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/comment5.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/comment_crlf_newline.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/blank_lines_lower_bound/1.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/brace_style/fn_always_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/brace_style/fn_prefer_same_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/brace_style/fn_same_line_where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/brace_style/item_always_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/brace_style/item_prefer_same_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/brace_style/item_same_line_where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/comment_width/above.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/comment_width/below.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/comment_width/ignore.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/condense_wildcard_suffixes/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/condense_wildcard_suffixes/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/control_brace_style/always_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/control_brace_style/always_same_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/control_brace_style/closing_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/disable_all_formatting/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/disable_all_formatting/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/empty_item_single_line/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/empty_item_single_line/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/error_on_line_overflow/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/fn_args_density/compressed.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/fn_args_density/tall.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/fn_args_density/vertical.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/fn_single_line/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/fn_single_line/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/force_explicit_abi/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/force_explicit_abi/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/force_multiline_block/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/force_multiline_block/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/format_strings/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/format_strings/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/hard_tabs/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/hard_tabs/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/imports_layout/merge_mixed.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_args.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_array.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_chain.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_generic.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_struct_lit.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_trailing_comma_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/block_where_pred.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/default.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/rfc_where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_args.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_array.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_chain.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_generics.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_struct_lit.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_trailing_comma.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/indent_style/visual_where_pred.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/match_arm_blocks/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/match_arm_blocks/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/match_block_trailing_comma/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/match_block_trailing_comma/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/merge_derives/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/normalize_comments/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/normalize_comments/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/remove_nested_parens/remove_nested_parens.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_impl_items/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_impl_items/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_imports/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_imports/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_modules/dolor/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_modules/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_modules/ipsum/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_modules/lorem/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_modules/sit/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/reorder_modules/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/skip_children/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/space_before_colon/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/spaces_around_ranges/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/spaces_around_ranges/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/struct_field_align_threshold/20.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/struct_lit_single_line/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/tab_spaces/2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/tab_spaces/4.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/trailing_comma/always.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/trailing_comma/never.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/trailing_comma/vertical.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/type_punctuation_density/compressed.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/type_punctuation_density/wide.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/use_field_init_shorthand/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/use_field_init_shorthand/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/use_small_heuristics/max.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/use_try_shorthand/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/use_try_shorthand/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/where_single_line/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/wrap_comments/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/configs/wrap_comments/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/control-brace-style-always-next-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/control-brace-style-always-same-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/doc-comment-with-example.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/doc.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/dyn_trait.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/else-if-brace-style-always-next-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/else-if-brace-style-always-same-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/else-if-brace-style-closing-next-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/empty_file.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/enum.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/existential_type.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/expr-block.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/expr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/extern.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/extern_not_explicit.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-1.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-3.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-4.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-5.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-6.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/file-lines-item.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom-2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom-3.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom-4.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom-6.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom-7.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom-8.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-custom.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-simple.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn-single-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn_args_density-vertical.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/fn_args_indent-block.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/hard-tabs.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/hello.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/hello2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/if_while_or_patterns.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/immovable_generators.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/impls.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/imports-reorder-lines-and-items.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/imports-reorder-lines.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/imports-reorder.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/imports.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/imports_block_indent.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1021.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1049.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1111.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1120.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1124.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1127.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1158.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1177.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1192.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1210/a.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1210/b.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1210/c.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1210/d.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1210/e.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1211.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1216.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1239.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1278.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1350.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1366.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1468.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1693.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1800.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-1914.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2025.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2111.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2164.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2179.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2256.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2342.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2445.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2446.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2479.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2482/a.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2482/b.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2482/c.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2520.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2523.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2582.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2641.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2644.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2728.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2761.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2794.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-2863.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-447.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-510.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-811.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-850.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-855.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-913.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-945.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/issue-977.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/item-brace-style-always-next-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/item-brace-style-prefer-same-line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/item-brace-style-same-line-where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/label_break.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/large-block.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/large_vec.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/lazy_static.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/long-match-arms-brace-newline.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/long_field_access.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/loop.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/macro_not_expr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/macro_rules.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/macros.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/markdown-comment-with-options.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/markdown-comment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/match-block-trailing-comma.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/match-nowrap-trailing-comma.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/match-nowrap.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/match.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/max-line-length-in-chars.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/merge_imports.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/mod-1.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/mod-2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/mod_skip_child.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/multiple.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nested-if-else.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nested_skipped/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/mod2a.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/mod2b.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/mod2c.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/mymod1/mod3a.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/submod2/a.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/nestedmod/submod2/mod.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/no_new_line_beginning.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/other.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/paren.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/paths.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/pattern-condense-wildcards.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/pattern.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/pub-restricted.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/remove_blank_lines.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/reorder-impl-items.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/single-line-if-else.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/soft-wrapping.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/space-not-before-newline.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/spaces-around-ranges.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/static.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/string-lit-2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/string-lit.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/string_punctuation.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/struct-field-attributes.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/struct_lits.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/struct_lits_multiline.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/struct_lits_visual.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/struct_lits_visual_multiline.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/struct_tuple_visual.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/structs.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/trailing-comma-never.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/trailing_commas.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/trait.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/try-conversion.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/tuple.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/type-ascription.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/type.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/type_alias.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/unions.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/visibility.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/where-clause-rfc.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/where-clause.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/source/width-heuristics.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/array_comment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/assignment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/associated-items.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/associated-types-bounds-wrapping.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/associated_type_defaults.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/async_closure.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/attrib-block-expr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/attrib-extern-crate.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/attrib.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/big-impl-block.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/big-impl-visual.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/binary-expr.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/break-and-continue.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/catch.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/chains-visual.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/chains.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/closure-block-inside-macro.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/closure.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment-inside-const.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment-not-disappear.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment2.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment3.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment4.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment5.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comment_crlf_newline.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/comments-fn.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/blank_lines_lower_bound/1.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/brace_style/fn_always_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/brace_style/fn_prefer_same_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/brace_style/fn_same_line_where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/brace_style/item_always_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/brace_style/item_prefer_same_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/brace_style/item_same_line_where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/combine_control_expr/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/combine_control_expr/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/comment_width/above.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/comment_width/below.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/comment_width/ignore.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/condense_wildcard_suffixes/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/condense_wildcard_suffixes/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/control_brace_style/always_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/control_brace_style/always_same_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/control_brace_style/closing_next_line.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/disable_all_formatting/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/disable_all_formatting/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/empty_item_single_line/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/empty_item_single_line/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/error_on_line_overflow/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/error_on_unformatted/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/fn_args_density/compressed.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/fn_args_density/tall.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/fn_args_density/vertical.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/fn_single_line/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/fn_single_line/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/force_explicit_abi/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/force_explicit_abi/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/force_multiline_block/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/force_multiline_block/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/format_strings/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/format_strings/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/hard_tabs/false.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/hard_tabs/true.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/imports_indent/block.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/imports_layout/horizontal_vertical.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/imports_layout/merge_mixed.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/imports_layout/mixed.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_args.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_array.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_chain.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_generic.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_struct_lit.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_tab_spaces_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_trailing_comma_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/block_where_pred.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/default.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/rfc_control.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/rfc_where.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_args.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_array.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_call.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_chain.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_generics.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_struct_lit.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_trailing_comma.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/indent_style/visual_where_pred.rs
[01:02:09]  WARN 2018-07-28T11:22:25Z: cargo::sources::path:   found /checkout/src/tools/rustfmt/tests/target/configs/match_arm_blocks/false.rs
---
[01:08:08]    Compiling cargo v0.30.0 (file:///checkout/src/tools/cargo)
[01:08:12]    Compiling rustc-ap-rustc_errors v209.0.0
[01:15:08]    Compiling racer v2.1.2
[01:16:56]    Compiling rls-vfs v0.4.6
[01:17:05] warning[E0502]: cannot borrow `packages.0` as mutable because it is also borrowed as immutable
[01:17:05]     |
[01:17:05] 435 |         match packages {
[01:17:05]     |               -------- immutable borrow occurs here
[01:17:05]     |               -------- immutable borrow occurs here
[01:17:05] 436 |             Some(ref mut packages) if packages.len() == 1 => {
[01:17:05]     |                  ^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:17:05] ...
[01:17:05] 439 |             _ => PackageArg::All,
[01:17:05]     |
[01:17:05]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
[01:17:05]             It represents potential unsoundness in your code.
[01:17:05]             This warning will become a hard error in the future.
[01:17:05]             This warning will become a hard error in the future.
[01:17:05] 
[01:17:06] warning[E0502]: cannot borrow `self.target_dir.0.0` as mutable because it is also borrowed as immutable
[01:17:06]    --> tools/rls/src/config.rs:246:40
[01:17:06]     |
[01:17:06] 244 |         match self.target_dir {
[01:17:06]     |               --------------- immutable borrow occurs here
[01:17:06] 245 |             // We require an absolute path, so adjust a relative one if it's passed.
[01:17:06] 246 |             Inferrable::Specified(Some(ref mut path)) if path.is_relative() => {
[01:17:06]     |                                        ^^^^^^^^^^^^ mutable borrow occurs here
[01:17:06] 249 |             _ => {},
[01:17:06]     |             - borrow later used here
[01:17:06]     |
[01:17:06]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
---

[01:21:32] travis_time:end:stage2-rustdoc:start=1532777951078083440,finish=1532778107724447684,duration=156646364244

[01:21:32]    Compiling rls v0.129.0 (file:///checkout/src/tools/rls)
[01:21:40] warning[E0502]: cannot borrow `packages.0` as mutable because it is also borrowed as immutable
[01:21:40]     |
[01:21:40] 435 |         match packages {
[01:21:40]     |               -------- immutable borrow occurs here
[01:21:40]     |               -------- immutable borrow occurs here
[01:21:40] 436 |             Some(ref mut packages) if packages.len() == 1 => {
[01:21:40]     |                  ^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:21:40] ...
[01:21:40] 439 |             _ => PackageArg::All,
[01:21:40]     |
[01:21:40]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
[01:21:40]             It represents potential unsoundness in your code.
[01:21:40]             This warning will become a hard error in the future.
[01:21:40]             This warning will become a hard error in the future.
[01:21:40] 
[01:21:40] warning[E0502]: cannot borrow `self.target_dir.0.0` as mutable because it is also borrowed as immutable
[01:21:40]    --> tools/rls/src/config.rs:246:40
[01:21:40]     |
[01:21:40] 244 |         match self.target_dir {
[01:21:40]     |               --------------- immutable borrow occurs here
[01:21:40] 245 |             // We require an absolute path, so adjust a relative one if it's passed.
[01:21:40] 246 |             Inferrable::Specified(Some(ref mut path)) if path.is_relative() => {
[01:21:40]     |                                        ^^^^^^^^^^^^ mutable borrow occurs here
[01:21:40] 249 |             _ => {},
[01:21:40]     |             - borrow later used here
[01:21:40]     |
[01:21:40]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
---
[01:23:33] test actions::diagnostics::diagnostic_suggestion_test::suggest_use_when_cannot_find_type ... ok
[\"method\":\"window/progress\",\"params\":{\"id\":\"progress_5\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_5\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_4\",\"title\":\"Indexing\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
[01:23:33]             "progress"
[01:23:33]         ]
[01:23:33]    "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_1\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_0\",\"title\":\"Indexing\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]             "bin_lib/tests/tests.rs",
[01:23:33]             "unused variable: `unused_var`"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
---
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-1654\",\"rls.deglobImports-1654\"]}}}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_11\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_11\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_10\",\"title\":\"Indexing\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]             "progress",
[01:23:33]             "deglob"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]         contains: [
[01:23:33]             "progress",
[01:23:33]             "deglob"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
---
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]             "\"message\":\"cannot find struct, variant or union type `Bar` in this scope"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: ct_messages:
[01:23:33]   results: [
[01:23:33]   results: [
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-1654\",\"rls.deglobImports-1654\"]}}}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_25\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_25\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_24\",\"title\":\"Indexing\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]             "progress",
[01:23:33]             "infer_bin"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]         contains: [
[01:23:33]             "progress",
[01:23:33]             "infer_bin"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
---
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]             "struct is never used: `UnusedCustomBin`"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]         contains: [
[01:23:33]             "progress",
[01:23:33]             "\"done\":true"
[01:23:33]         ]
[01:23:33]     }
[01:23:33] ]
[01:23:33] thread 'test::test_infer_custom_bin' panicked at 'assertion failed: `(left == right)`
[01:23:33]   left: `6`,
[01:23:33]  right: dow/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]             "unused variable: `bin_name"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
[01:23:33]         contains: [
[01:23:33]             "unused variable: `bin_name"
[01:23:33]     },
[01:23:33]     ExpectedMessage {
[01:23:33]         id: None,
[01:23:33]         contains: [
---
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvideCommandProvider\":{\"commands\":[\"rls.applySuggestion-1654\",\"rls.deglobImports-1654\"]}}}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_51\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_51\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_50\",\"title\":\"Indexing\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
[01:23:33]             "progress",
[01:23:33:\"progress_53\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_53\",\"title\":\"Building\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_52\",\"title\":\"Indexing\"}}",
[01:23:33]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:23:33] ],
[01:23:33]   expected: [
[01:23:33]     ExpectedMessage {
[01:23:33]         id: Some(
---
travis_time:end:03241408:start=1532778231444335951,finish=1532778231450781254,duration=6445303
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01b492c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18b4ca8b
travis_time:start:18b4ca8b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f84e72b
$ dmesg | grep -i kill
