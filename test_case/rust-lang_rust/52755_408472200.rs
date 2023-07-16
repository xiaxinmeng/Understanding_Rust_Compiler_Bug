plain
[01:17:59]    Compiling cargo v0.28.0
[01:17:59]    Compiling cargo v0.30.0 (file:///checkout/src/tools/cargo)
[01:25:46]    Compiling racer v2.1.2
[01:27:31]    Compiling rls-vfs v0.4.6
[01:27:38] error: unused imports: `json_internal`, `json`
[01:27:38]   --> tools/rls/src/server/message.rs:18:24
[01:27:38]    |
[01:27:38] 18 | use serde_json::{self, json, json_internal};
[01:27:38]    |                        ^^^^  ^^^^^^^^^^^^^
[01:27:38]    = note: `-D unused-imports` implied by `-D warnings`
[01:27:38] 
[01:27:40] error: aborting due to previous error
[01:27:40] 
[01:27:40] 
[01:27:40] error: Could not compile `rls`.
[01:27:40] 
[01:27:40] Caused by:
[01:27:40]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name rls tools/rls/src/main.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 --cfg feature="clippy" --cfg feature="clippy_lints" --cfg feature="default" -C metadata=7316608ed32e692f -C extra-filename=-7316608ed32e692f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern cargo=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcargo-82866fb49df6f2c8.rlib --extern cargo_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcargo_metadata-acbac866a3a17caa.rlib --extern clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a7c2feb5e29bfad4.rlib --extern crossbeam_channel=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_channel-ae47ded5e5ddac88.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-ab606e34bf676104.rlib --extern failure=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfailure-c91eda60b92c826f.rlib --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-e1e0a5dd28a1370a.rlib --extern jsonrpc_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libjsonrpc_core-f14e83002f89b3e5.rlib --extern languageserver_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblanguageserver_types-2792042ba5a12592.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblazy_static-16352ca52197493b.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblog-30406620e8787835.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-a828d3bbf0493f76.rlib --extern ordslice=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libordslice-8e14c7436f02560e.rlib --extern racer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libracer-f7ba2ca069f0dc9f.rlib --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librayon-aeaaa0a5b14394bb.rlib --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-431d6b808d268ed2.rlib --extern rls_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_analysis-67308cf1cfe57988.rlib --extern rls_blacklist=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_blacklist-d2a90cd837503a47.rlib --extern rls_data=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_data-2f90042d93c8e6e5.rlib --extern rls_rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_rustc-92d2862f3b592994.rlib --extern rls_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_span-df7f3fb7d1f0167a.rlib --extern rls_vfs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_vfs-7a6386ce930eac4a.rlib --extern rustfmt_nightly=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustfmt_nightly-6837c5d2f8f0bf23.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-58141076a80f681b.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-7ca48a6ccc017769.so --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-1f047ad146fdd8da.rlib --extern url=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-f8c16dc48a4d0e23.rlib --extern walkdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libwalkdir-eea44e6a7cdd1b16.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-ede6b7d2a17ef20e/out/lib -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-ba701a1ee4991c9a/out/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-c73017c57a351928/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/miniz-sys-ef244e975de97174/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-db34522498c386e4/out/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-77aa4e01a9e28e64/out/lib` (exit code: 1)
[01:27:40] expected success, got: exit code: 101
[01:27:40] travis_fold:end:stage2-rls

[01:27:40] travis_time:end:stage2-rls:start=1532707890776417730,finish=1532708819816249371,duration=929039831641
---
[01:33:45] test shape::test::shape_block_indent_without_alignment ... ok
[01:33:45] test string::test::big_whitespace ... ok
[01:33:45] test string::test::should_break_forward ... ok
[01:33:45] test string::test::issue343 ... ok
[01:33:45] test string::test::nothing_to_break ... ok
[01:33:45] test string::test::should_break_on_punctuation ... ok
[01:33:45] test string::test::should_break_on_whitespace ... ok
[01:33:45] test string::test::significant_whitespaces ... ok
[01:33:45] test test::format_lines_errors_are_reported_with_tabs ... ok
[01:33:45] test test::coverage_tests ... ok
[01:33:45] test test::checkstyle_test ... ok
[01:33:45] test test::modified_test ... ok
---
[01:34:57] failures:
[01:34:57] 
[01:34:57] ---- [ui] ui/redundant_field_names.rs stdout ----
[01:34:57] normalized stderr:
[01:34:57] error[E0464]: multiple matching crates for `derive_new`
[01:34:57]  --> $DIR/redundant_field_names.rs:6:1
[01:34:57] 6 | extern crate derive_new;
[01:34:57]   | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:57]   |
[01:34:57]   = note: candidates:
[01:34:57]   = note: candidates:
[01:34:57]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:34:57]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:34:57] error[E0463]: can't find crate for `derive_new`
[01:34:57]  --> $DIR/redundant_field_names.rs:6:1
[01:34:57]   |
[01:34:57] 6 | extern crate derive_new;
---
[01:34:57] expected stderr:
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:34:9
[01:34:57]    |
[01:34:57] 34 |         gender: gender,
[01:34:57]    |         ^^^^^^^^^^^^^^ help: replace it with: `gender`
[01:34:57]    = note: `-D redundant-field-names` implied by `-D warnings`
[01:34:57] 
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:35:9
[01:34:57]   --> $DIR/redundant_field_names.rs:35:9
[01:34:57]    |
[01:34:57] 35 |         age: age,
[01:34:57]    |         ^^^^^^^^ help: replace it with: `age`
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:53:25
[01:34:57]    |
[01:34:57]    |
[01:34:57] 53 |     let _ = RangeFrom { start: start };
[01:34:57]    |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:54:23
[01:34:57]    |
[01:34:57]    |
[01:34:57] 54 |     let _ = RangeTo { end: end };
[01:34:57]    |                       ^^^^^^^^ help: replace it with: `end`
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:55:21
[01:34:57]    |
[01:34:57]    |
[01:34:57] 55 |     let _ = Range { start: start, end: end };
[01:34:57]    |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:55:35
[01:34:57]    |
[01:34:57]    |
[01:34:57] 55 |     let _ = Range { start: start, end: end };
[01:34:57]    |                                   ^^^^^^^^ help: replace it with: `end`
[01:34:57] error: redundant field names in struct initialization
[01:34:57]   --> $DIR/redundant_field_names.rs:57:32
[01:34:57]    |
[01:34:57]    |
[01:34:57] 57 |     let _ = RangeToInclusive { end: end };
[01:34:57]    |                                ^^^^^^^^ help: replace it with: `end`
[01:34:57] error: aborting due to 7 previous errors
[01:34:57] 
[01:34:57] 
[01:34:57] 
[01:34:57] 
[01:34:57] diff of stderr:
[01:34:57] 
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:34:9
[01:34:57] -   |
[01:34:57] -34 |         gender: gender,
[01:34:57] -   |         ^^^^^^^^^^^^^^ help: replace it with: `gender`
[01:34:57] -   = note: `-D redundant-field-names` implied by `-D warnings`
[01:34:57] -   = note: `-D redundant-field-names` implied by `-D warnings`
[01:34:57] +error[E0464]: multiple matching crates for `derive_new`
[01:34:57] + --> $DIR/redundant_field_names.rs:6:1
[01:34:57] +  |
[01:34:57] +6 | extern crate derive_new;
[01:34:57] +  | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:57] +  |
[01:34:57] +  = note: candidates:
[01:34:57] +          crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:34:57] +          crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:35:9
[01:34:57] -   |
[01:34:57] -   |
[01:34:57] -35 |         age: age,
[01:34:57] -   |         ^^^^^^^^ help: replace it with: `age`
[01:34:57] +error[E0463]: can't find crate for `derive_new`
[01:34:57] + --> $DIR/redundant_field_names.rs:6:1
[01:34:57] +  |
[01:34:57] +6 | extern crate derive_new;
[01:34:57] +  | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:53:25
[01:34:57] -   |
[01:34:57] -   |
[01:34:57] -53 |     let _ = RangeFrom { start: start };
[01:34:57] -   |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:34:57]  
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:54:23
[01:34:57] -   |
[01:34:57] -   |
[01:34:57] -54 |     let _ = RangeTo { end: end };
[01:34:57] -   |                       ^^^^^^^^ help: replace it with: `end`
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:55:21
[01:34:57] -   |
[01:34:57] -   |
[01:34:57] -55 |     let _ = Range { start: start, end: end };
[01:34:57] -   |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:55:35
[01:34:57] -   |
[01:34:57] -   |
[01:34:57] -55 |     let _ = Range { start: start, end: end };
[01:34:57] -   |                                   ^^^^^^^^ help: replace it with: `end`
[01:34:57] -error: redundant field names in struct initialization
[01:34:57] -  --> $DIR/redundant_field_names.rs:57:32
[01:34:57] -   |
[01:34:57] -   |
[01:34:57] -57 |     let _ = RangeToInclusive { end: end };
[01:34:57] -   |                                ^^^^^^^^ help: replace it with: `end`
[01:34:57] -error: aborting due to 7 previous errors
[01:34:57] -
[01:34:57] +Some errors occurred: E0463, E0464.
[01:34:57] +For more information about an error, try `rustc --explain E0463`.
---
[01:34:57] 
[01:34:57] ------------------------------------------
[01:34:57] stderr:
[01:34:57] ------------------------------------------
[01:34:57] error[E0464]: multiple matching crates for `derive_new`
[01:34:57]   |
[01:34:57] 6 | extern crate derive_new;
[01:34:57]   | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:34:57]   |
[01:34:57]   |
[01:34:57]   = note: candidates:
[01:34:57]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:34:57]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:34:57] error[E0463]: can't find crate for `derive_new`
[01:34:57]  --> tests/ui/redundant_field_names.rs:6:1
[01:34:57]   |
[01:34:57] 6 | extern crate derive_new;
---
[01:34:57] Verifying status of rust-by-example...
[01:34:57] Verifying status of rls...
[01:34:57] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:34:57] 
[01:34:57] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:34:57] 
[01:34:57] If you do intend to update 'rls', please check the error messages above and
[01:34:57] commit another update.
[01:34:57] 
[01:34:57] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:34:57] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:34:57] proper steps.
4494580 ./obj
4494544 ./obj/build
3900656 ./obj/build/x86_64-unknown-linux-gnu
1256788 ./obj/build/x86_64-unknown-linux-gnu/llvm
---
travis_time:end:17863e1a:start=1532709258386935986,finish=1532709258395509033,duration=8573047
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00010bf8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03e72e40
travis_time:start:03e72e40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06a0c269
$ dmesg | grep -i kill
