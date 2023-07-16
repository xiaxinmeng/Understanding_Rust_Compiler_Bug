plain
[01:06:32]    Compiling cargo v0.30.0 (file:///checkout/src/tools/cargo)
[01:13:03]    Compiling racer v2.1.2
[01:14:07]    Compiling rls-vfs v0.4.6
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]   --> tools/rls/src/actions/work_pool.rs:14:27
[01:14:12] 14 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[01:14:12]    |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/cargo.rs:271:14
[01:14:12]     |
[01:14:12] 271 |         ws: &Workspace,
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/cargo.rs:311:36
[01:14:12]     |
[01:14:12]     |
[01:14:12] 311 |         let only_primary = |unit: &Unit| self.is_primary_crate(unit.pkg.package_id());
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/cargo.rs:308:25
[01:14:12]     |
[01:14:12]     |
[01:14:12] 308 |     fn init(&self, cx: &Context, unit: &Unit) {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/cargo.rs:308:41
[01:14:12]     |
[01:14:12]     |
[01:14:12] 308 |     fn init(&self, cx: &Context, unit: &Unit) {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/cargo.rs:318:36
[01:14:12]     |
[01:14:12]     |
[01:14:12] 318 |     fn force_rebuild(&self, unit: &Unit) -> bool {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/rustc.rs:140:44
[01:14:12]     |
[01:14:12]     |
[01:14:12] 140 | fn clippy_after_parse_callback(state: &mut ::rustc_driver::driver::CompileState) {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:103:36
[01:14:12]     |
[01:14:12]     |
[01:14:12] 103 |         let null_filter = |_unit: &Unit| true;
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:102:44
[01:14:12]     |
[01:14:12]     |
[01:14:12] 102 |     crate fn emplace_dep(&mut self, unit: &Unit, cx: &Context) -> CargoResult<()> {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:102:55
[01:14:12]     |
[01:14:12]     |
[01:14:12] 102 |     crate fn emplace_dep(&mut self, unit: &Unit, cx: &Context) -> CargoResult<()> {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:117:21
[01:14:12]     |
[01:14:12]     |
[01:14:12] 117 |         Filter: Fn(&Unit) -> bool,
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:112:16
[01:14:12]     |
---
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:572:25
[01:14:12]     |
[01:14:12] 572 | fn key_from_unit(unit: &Unit) -> UnitKey {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/build/plan.rs:589:27
[01:14:12]     |
[01:14:12]     |
[01:14:12] 589 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[01:14:12]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]    --> tools/rls/src/config.rs:300:45
[01:14:12]     |
[01:14:12] 300 |         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
[01:14:12] 
[01:14:12] error: hidden lifetime parameters in types are deprecated
[01:14:12]   --> tools/rls/src/lsp_data.rs:50:27
[01:14:12]    |
---
[01:14:13] 
[01:14:13] error: Could not compile `rls`.
[01:14:13] 
[01:14:13] Caused by:
[01:14:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name rls tools/rls/src/main.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 --cfg feature="clippy" --cfg feature="clippy_lints" --cfg feature="default" -C metadata=7316608ed32e692f -C extra-filename=-7316608ed32e692f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern cargo=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcargo-82866fb49df6f2c8.rlib --extern cargo_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcargo_metadata-acbac866a3a17caa.rlib --extern clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-a7c2feb5e29bfad4.rlib --extern crossbeam_channel=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_channel-ae47ded5e5ddac88.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-ab606e34bf676104.rlib --extern failure=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfailure-c91eda60b92c826f.rlib --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-e1e0a5dd28a1370a.rlib --extern jsonrpc_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libjsonrpc_core-f14e83002f89b3e5.rlib --extern languageserver_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblanguageserver_types-2792042ba5a12592.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblazy_static-16352ca52197493b.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liblog-30406620e8787835.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-a828d3bbf0493f76.rlib --extern ordslice=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libordslice-8e14c7436f02560e.rlib --extern racer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libracer-f7ba2ca069f0dc9f.rlib --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librayon-aeaaa0a5b14394bb.rlib --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-431d6b808d268ed2.rlib --extern rls_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_analysis-67308cf1cfe57988.rlib --extern rls_blacklist=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_blacklist-d2a90cd837503a47.rlib --extern rls_data=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_data-2f90042d93c8e6e5.rlib --extern rls_rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_rustc-92d2862f3b592994.rlib --extern rls_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_span-df7f3fb7d1f0167a.rlib --extern rls_vfs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librls_vfs-7a6386ce930eac4a.rlib --extern rustfmt_nightly=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustfmt_nightly-6837c5d2f8f0bf23.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-58141076a80f681b.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-7ca48a6ccc017769.so --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-1f047ad146fdd8da.rlib --extern url=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/liburl-f8c16dc48a4d0e23.rlib --extern walkdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libwalkdir-eea44e6a7cdd1b16.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-ede6b7d2a17ef20e/out/lib -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-ba701a1ee4991c9a/out/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-c73017c57a351928/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/miniz-sys-ef244e975de97174/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-db34522498c386e4/out/lib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-77aa4e01a9e28e64/out/lib` (exit code: 1)
[01:14:13] expected success, got: exit code: 101
[01:14:13] travis_fold:end:stage2-rls

[01:14:13] travis_time:end:stage2-rls:start=1532684173085058281,finish=1532684928635409217,duration=755550350936
---
[01:19:23] test shape::test::indent_to_string_hard_tabs ... ok
[01:19:23] test shape::test::shape_block_indent_without_alignment ... ok
[01:19:23] test shape::test::shape_visual_indent ... ok
[01:19:23] test string::test::issue343 ... ok
[01:19:23] test string::test::big_whitespace ... ok
[01:19:23] test string::test::nothing_to_break ... ok
[01:19:23] test string::test::should_break_forward ... ok
[01:19:23] test string::test::should_break_on_whitespace ... ok
[01:19:23] test string::test::should_break_on_punctuation ... ok
[01:19:23] test string::test::significant_whitespaces ... ok
[01:19:23] test test::checkstyle_test ... ok
[01:19:23] test test::coverage_tests ... ok
[01:19:23] test test::modified_test ... ok
[01:19:23] test test::format_lines_errors_are_reported_with_tabs ... ok
---
[01:20:28] 
[01:20:28] error: redundant field names in struct initialization
[01:20:28]   --> $DIR/redundant_field_names.rs:35:9
[01:20:28]    |
[01:20:28] 35 |         age: age,
[01:20:28]    |         ^^^^^^^^ help: replace it with: `age`
[01:20:28] error: redundant field names in struct initialization
[01:20:28]   --> $DIR/redundant_field_names.rs:53:25
[01:20:28]    |
[01:20:28]    |
[01:20:28] 53 |     let _ = RangeFrom { start: start };
[01:20:28]    |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:20:28] error: redundant field names in struct initialization
[01:20:28]   --> $DIR/redundant_field_names.rs:54:23
[01:20:28]    |
[01:20:28]    |
[01:20:28] 54 |     let _ = RangeTo { end: end };
[01:20:28]    |                       ^^^^^^^^ help: replace it with: `end`
[01:20:28] error: redundant field names in struct initialization
[01:20:28]   --> $DIR/redundant_field_names.rs:55:21
[01:20:28]    |
[01:20:28]    |
[01:20:28] 55 |     let _ = Range { start: start, end: end };
[01:20:28]    |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:20:28] error: redundant field names in struct initialization
[01:20:28]   --> $DIR/redundant_field_names.rs:55:35
[01:20:28]    |
[01:20:28]    |
[01:20:28] 55 |     let _ = Range { start: start, end: end };
[01:20:28]    |                                   ^^^^^^^^ help: replace it with: `end`
[01:20:28] error: redundant field names in struct initialization
[01:20:28]   --> $DIR/redundant_field_names.rs:57:32
[01:20:28]    |
[01:20:28]    |
[01:20:28] 57 |     let _ = RangeToInclusive { end:

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:01755226
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
