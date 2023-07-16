plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 72 tests
...F.F.............FF.FiFFF..F..F...FFFFFFFF.FFF.F.FFFF.FF.FFF..F.F...F.

---- [ui] tests/ui-fulldeps/gated-plugin.rs stdout ----


error: auxiliary build of "/checkout/tests/ui-fulldeps/auxiliary/empty-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/auxiliary/empty-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
error: crate `rustc_driver` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `tracing` required to be available in rlib format, but was not found in this form

error: crate `tracing_core` required to be available in rlib format, but was not found in this form

error: crate `once_cell` required to be available in rlib format, but was not found in this form

error: crate `pin_project_lite` required to be available in rlib format, but was not found in this form

error: crate `cfg_if` required to be available in rlib format, but was not found in this form

error: crate `rustc_plugin_impl` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_lint` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_middle` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `bitflags` required to be available in rlib format, but was not found in this form

error: crate `rustc_data_structures` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_index` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `arrayvec` required to be available in rlib format, but was not found in this form

error: crate `smallvec` required to be available in rlib format, but was not found in this form

error: crate `rustc_serialize` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `thin_vec` required to be available in rlib format, but was not found in this form

error: crate `indexmap` required to be available in rlib format, but was not found in this form

error: crate `hashbrown` required to be available in rlib format, but was not found in this form

error: crate `ahash` required to be available in rlib format, but was not found in this form

error: crate `getrandom` required to be available in rlib format, but was not found in this form

error: crate `libc` required to be available in rlib format, but was not found in this form

error: crate `rustc_hash` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `jobserver` required to be available in rlib format, but was not found in this form

error: crate `rustc_graphviz` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `stable_deref_trait` required to be available in rlib format, but was not found in this form

error: crate `ena` required to be available in rlib format, but was not found in this form

error: crate `log` required to be available in rlib format, but was not found in this form

error: crate `measureme` required to be available in rlib format, but was not found in this form

error: crate `memmap2` required to be available in rlib format, but was not found in this form

error: crate `perf_event_open_sys` required to be available in rlib format, but was not found in this form

error: crate `parking_lot` required to be available in rlib format, but was not found in this form

error: crate `instant` required to be available in rlib format, but was not found in this form

error: crate `lock_api` required to be available in rlib format, but was not found in this form

error: crate `scopeguard` required to be available in rlib format, but was not found in this form

error: crate `parking_lot_core` required to be available in rlib format, but was not found in this form

error: crate `tempfile` required to be available in rlib format, but was not found in this form

error: crate `remove_dir_all` required to be available in rlib format, but was not found in this form

error: crate `fastrand` required to be available in rlib format, but was not found in this form

error: crate `stacker` required to be available in rlib format, but was not found in this form

error: crate `psm` required to be available in rlib format, but was not found in this form

error: crate `rustc_span` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_arena` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `unicode_width` required to be available in rlib format, but was not found in this form

error: crate `md5` required to be available in rlib format, but was not found in this form

error: crate `digest` required to be available in rlib format, but was not found in this form

error: crate `crypto_common` required to be available in rlib format, but was not found in this form

error: crate `generic_array` required to be available in rlib format, but was not found in this form

error: crate `typenum` required to be available in rlib format, but was not found in this form

error: crate `block_buffer` required to be available in rlib format, but was not found in this form

error: crate `sha1` required to be available in rlib format, but was not found in this form

error: crate `cpufeatures` required to be available in rlib format, but was not found in this form

error: crate `sha2` required to be available in rlib format, but was not found in this form

error: crate `scoped_tls` required to be available in rlib format, but was not found in this form

error: crate `rustc_hir` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_ast` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_lexer` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `unicode_xid` required to be available in rlib format, but was not found in this form

error: crate `unic_emoji_char` required to be available in rlib format, but was not found in this form

error: crate `unic_char_property` required to be available in rlib format, but was not found in this form

error: crate `unic_char_range` required to be available in rlib format, but was not found in this form

error: crate `unic_ucd_version` required to be available in rlib format, but was not found in this form

error: crate `unic_common` required to be available in rlib format, but was not found in this form

error: crate `memchr` required to be available in rlib format, but was not found in this form

error: crate `rustc_error_messages` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `fluent_syntax` required to be available in rlib format, but was not found in this form

error: crate `thiserror` required to be available in rlib format, but was not found in this form

error: crate `icu_provider_adapters` required to be available in rlib format, but was not found in this form

error: crate `icu_provider` required to be available in rlib format, but was not found in this form

error: crate `yoke` required to be available in rlib format, but was not found in this form

error: crate `zerofrom` required to be available in rlib format, but was not found in this form

error: crate `serde` required to be available in rlib format, but was not found in this form

error: crate `writeable` required to be available in rlib format, but was not found in this form

error: crate `zerovec` required to be available in rlib format, but was not found in this form

error: crate `icu_locid` required to be available in rlib format, but was not found in this form

error: crate `litemap` required to be available in rlib format, but was not found in this form

error: crate `tinystr` required to be available in rlib format, but was not found in this form

error: crate `intl_memoizer` required to be available in rlib format, but was not found in this form

error: crate `unic_langid` required to be available in rlib format, but was not found in this form

error: crate `unic_langid_impl` required to be available in rlib format, but was not found in this form

error: crate `unic_langid_macros` required to be available in rlib format, but was not found in this form

error: crate `type_map` required to be available in rlib format, but was not found in this form

error: crate `fluent_bundle` required to be available in rlib format, but was not found in this form

error: crate `self_cell` required to be available in rlib format, but was not found in this form

error: crate `intl_pluralrules` required to be available in rlib format, but was not found in this form

error: crate `fluent_langneg` required to be available in rlib format, but was not found in this form

error: crate `icu_list` required to be available in rlib format, but was not found in this form

error: crate `regex_automata` required to be available in rlib format, but was not found in this form

error: crate `rustc_baked_icu_data` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_target` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_abi` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rand` required to be available in rlib format, but was not found in this form

error: crate `rand_core` required to be available in rlib format, but was not found in this form

error: crate `rand_chacha` required to be available in rlib format, but was not found in this form

error: crate `ppv_lite86` required to be available in rlib format, but was not found in this form

error: crate `rand_xoshiro` required to be available in rlib format, but was not found in this form

error: crate `serde_json` required to be available in rlib format, but was not found in this form

error: crate `itoa` required to be available in rlib format, but was not found in this form

error: crate `ryu` required to be available in rlib format, but was not found in this form

error: crate `rustc_feature` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `odht` required to be available in rlib format, but was not found in this form

error: crate `rustc_query_system` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_session` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_errors` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_lint_defs` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `termcolor` required to be available in rlib format, but was not found in this form

error: crate `annotate_snippets` required to be available in rlib format, but was not found in this form

error: crate `rustc_ast_pretty` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_type_ir` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `termize` required to be available in rlib format, but was not found in this form

error: crate `rustc_fs_util` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `getopts` required to be available in rlib format, but was not found in this form

error: crate `rustc_attr` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `polonius_engine` required to be available in rlib format, but was not found in this form

error: crate `datafrog` required to be available in rlib format, but was not found in this form

error: crate `either` required to be available in rlib format, but was not found in this form

error: crate `gsgdt` required to be available in rlib format, but was not found in this form

error: crate `rustc_apfloat` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `chalk_ir` required to be available in rlib format, but was not found in this form

error: crate `rustc_trait_selection` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_infer` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_parse_format` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_transmute` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `unicode_security` required to be available in rlib format, but was not found in this form

error: crate `unicode_normalization` required to be available in rlib format, but was not found in this form

error: crate `tinyvec` required to be available in rlib format, but was not found in this form

error: crate `tinyvec_macros` required to be available in rlib format, but was not found in this form

error: crate `unicode_script` required to be available in rlib format, but was not found in this form

error: crate `libloading` required to be available in rlib format, but was not found in this form

error: crate `rustc_metadata` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_expand` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_parse` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_ast_passes` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `itertools` required to be available in rlib format, but was not found in this form

error: crate `crossbeam_channel` required to be available in rlib format, but was not found in this form

error: crate `crossbeam_utils` required to be available in rlib format, but was not found in this form

error: crate `snap` required to be available in rlib format, but was not found in this form

error: crate `rustc_hir_pretty` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_codegen_ssa` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `ar_archive_writer` required to be available in rlib format, but was not found in this form

error: crate `object` required to be available in rlib format, but was not found in this form

error: crate `object` required to be available in rlib format, but was not found in this form

error: crate `flate2` required to be available in rlib format, but was not found in this form

error: crate `crc32fast` required to be available in rlib format, but was not found in this form

error: crate `miniz_oxide` required to be available in rlib format, but was not found in this form

error: crate `adler` required to be available in rlib format, but was not found in this form

error: crate `cc` required to be available in rlib format, but was not found in this form

error: crate `regex` required to be available in rlib format, but was not found in this form

error: crate `regex_syntax` required to be available in rlib format, but was not found in this form

error: crate `aho_corasick` required to be available in rlib format, but was not found in this form

error: crate `pathdiff` required to be available in rlib format, but was not found in this form

error: crate `rustc_incremental` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_symbol_mangling` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `twox_hash` required to be available in rlib format, but was not found in this form

error: crate `static_assertions` required to be available in rlib format, but was not found in this form

error: crate `punycode` required to be available in rlib format, but was not found in this form

error: crate `rustc_demangle` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `thorin` required to be available in rlib format, but was not found in this form

error: crate `gimli` required to be available in rlib format, but was not found in this form

error: crate `fallible_iterator` required to be available in rlib format, but was not found in this form

error: crate `rustc_interface` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_query_impl` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_borrowck` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_mir_dataflow` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_const_eval` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_traits` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `chalk_solve` required to be available in rlib format, but was not found in this form

error: crate `petgraph` required to be available in rlib format, but was not found in this form

error: crate `fixedbitset` required to be available in rlib format, but was not found in this form

error: crate `tracing_subscriber` required to be available in rlib format, but was not found in this form

error: crate `tracing_log` required to be available in rlib format, but was not found in this form

error: crate `lazy_static` required to be available in rlib format, but was not found in this form

error: crate `matchers` required to be available in rlib format, but was not found in this form

error: crate `regex_automata` required to be available in rlib format, but was not found in this form

error: crate `ansi_term` required to be available in rlib format, but was not found in this form

error: crate `sharded_slab` required to be available in rlib format, but was not found in this form

error: crate `thread_local` required to be available in rlib format, but was not found in this form

error: crate `tracing_tree` required to be available in rlib format, but was not found in this form

error: crate `atty` required to be available in rlib format, but was not found in this form

error: crate `chalk_engine` required to be available in rlib format, but was not found in this form

error: crate `rustc_mir_build` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_passes` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_resolve` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_builtin_macros` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_ast_lowering` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_mir_transform` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_monomorphize` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_privacy` required to be available in rlib format, but was not found in this form
   |
   = help: try adding `extern crate rustc_driver;` at the top level of this crate

error: crate `rustc_hir_analysis` required to be available in rlib format, but was not found in this form
   |
