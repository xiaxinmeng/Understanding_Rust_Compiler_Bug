plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0616]: field `incremental` of struct `options::Options` is private
    |
812 |         self.incremental.is_some()
    |              ^^^^^^^^^^^ private field


error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/config.rs:813:21
    |
813 |             || self.debugging_opts.dump_dep_graph

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/config.rs:814:21
    |
    |
814 |             || self.debugging_opts.query_dep_graph


error[E0616]: field `remap_path_prefix` of struct `options::Options` is private
    |
    |
818 |         FilePathMapping::new(self.remap_path_prefix.clone())

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/config.rs:823:15
    |
    |
823 |         !self.debugging_opts.parse_only && // The file is just being parsed

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/config.rs:824:19
    |
    |
824 |             !self.debugging_opts.ls // The file is just being queried

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/config.rs:829:20
    |
---
    |
831 |             None => match self.optimize {
    |                                ^^^^^^^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
839 |         self.cg.symbol_mangling_version.unwrap_or(SymbolManglingVersion::Legacy)
    |              ^^ private field
error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/config.rs:1004:24
     |
     |
1004 |     for s in sess.opts.debugging_opts.sanitizer {

error[E0616]: field `debug_assertions` of struct `options::Options` is private
    --> compiler/rustc_session/src/config.rs:1009:18
     |
     |
1009 |     if sess.opts.debug_assertions {
     |                  ^^^^^^^^^^^^^^^^ private field

error[E0616]: field `crate_types` of struct `options::Options` is private
     |
     |
1012 |     if sess.opts.crate_types.contains(&CrateType::ProcMacro) {


error[E0616]: field `test` of struct `options::Options` is private
     |
1225 |     if sess.opts.test {
     |                  ^^^^ private field


error[E0616]: field `target_triple` of struct `options::Options` is private
     |
     |
1238 |         || Target::search(&opts.target_triple, sysroot),


error[E0616]: field `error_format` of struct `options::Options` is private
     |
1243 |             opts.error_format,
     |                  ^^^^^^^^^^^^ private field


error[E0616]: field `error_format` of struct `options::Options` is private
     |
     |
1252 |         early_warn(opts.error_format, &warning)


error[E0616]: field `error_format` of struct `options::Options` is private
     |
1257 |             opts.error_format,
     |                  ^^^^^^^^^^^^ private field


error[E0616]: field `json_future_incompat` of struct `options::Options` is private
    |
    |
283 |         if !self.opts.json_future_incompat {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:468:22
    |
    |
468 |         if self.opts.debugging_opts.print_type_sizes

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:469:26
    |
    |
469 |             || self.opts.debugging_opts.query_dep_graph

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:470:26
    |
    |
470 |             || self.opts.debugging_opts.dump_mir.is_some()

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:471:26
    |
    |
471 |             || self.opts.debugging_opts.unpretty.is_some()


error[E0616]: field `output_types` of struct `options::Options` is private
    |
    |
472 |             || self.opts.output_types.contains_key(&OutputType::Mir)

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:545:19
    |
    |
545 |         self.opts.debugging_opts.verbose

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:548:19
    |
    |
548 |         self.opts.debugging_opts.time_passes || self.opts.debugging_opts.time

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:548:59
    |
    |
548 |         self.opts.debugging_opts.time_passes || self.opts.debugging_opts.time

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:551:19
    |
    |
551 |         self.opts.debugging_opts.instrument_mcount

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:554:19
    |
    |
554 |         self.opts.debugging_opts.time_llvm_passes

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:557:19
    |
    |
557 |         self.opts.debugging_opts.meta_stats

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:560:19
    |
    |
560 |         self.opts.debugging_opts.asm_comments

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:563:19
    |
    |
563 |         self.opts.debugging_opts.verify_llvm_ir || option_env!("RUSTC_VERIFY_LLVM_IR").is_some()

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:566:19
    |
    |
566 |         self.opts.debugging_opts.print_llvm_passes

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:569:19
    |
    |
569 |         self.opts.debugging_opts.binary_dep_depinfo


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
600 |         match self.opts.cg.lto {
    |                         ^^ private field

error[E0616]: field `cli_forced_thinlto_off` of struct `options::Options` is private
    |
    |
614 |                 return if self.opts.cli_forced_thinlto_off {


error[E0616]: field `cli_forced_thinlto_off` of struct `options::Options` is private
    |
    |
630 |         if self.opts.cli_forced_thinlto_off {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:636:42
    |
    |
636 |         if let Some(enabled) = self.opts.debugging_opts.thinlto {

error[E0616]: field `optimize` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:652:25
    |
    |
652 |         match self.opts.optimize {
    |                         ^^^^^^^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
661 |         self.opts.cg.panic.unwrap_or(self.target.panic_strategy)
    |                   ^^ private field
error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:664:46
    |
    |
664 |         if let Some(fewer_names) = self.opts.debugging_opts.fewer_names {


error[E0616]: field `output_types` of struct `options::Options` is private
    |
    |
667 |             let more_names = self.opts.output_types.contains_key(&OutputType::LlvmAssembly)


error[E0616]: field `output_types` of struct `options::Options` is private
    |
    |
668 |                 || self.opts.output_types.contains_key(&OutputType::Bitcode)

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:670:30
    |
    |
670 |                 || self.opts.debugging_opts.sanitizer.intersects(SanitizerSet::ADDRESS | SanitizerSet::MEMORY);

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:676:19
    |
    |
676 |         self.opts.debugging_opts.unstable_options


error[E0616]: field `unstable_features` of struct `options::Options` is private
    |
    |
679 |         self.opts.unstable_features.is_nightly_build()

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:682:19
    |
    |
682 |         self.opts.debugging_opts.sanitizer.contains(SanitizerSet::CFI)


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
685 |         self.opts.cg.overflow_checks.unwrap_or(self.opts.debug_assertions)
    |                   ^^ private field
error[E0616]: field `debug_assertions` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:685:58
    |
    |
685 |         self.opts.cg.overflow_checks.unwrap_or(self.opts.debug_assertions)


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
695 |         let requested_features = self.opts.cg.target_feature.split(',');
    |                                            ^^ private field

error[E0616]: field `crate_types` of struct `options::Options` is private
    |
    |
702 |             || crate_type == None && self.opts.crate_types.contains(&CrateType::ProcMacro)


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
714 |         self.opts.cg.relocation_model.unwrap_or(self.target.relocation_model)
    |                   ^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
718 |         self.opts.cg.code_model.or(self.target.code_model)
    |                   ^^ private field
error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:722:19
    |
    |
722 |         self.opts.debugging_opts.tls_model.unwrap_or(self.target.tls_model)

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:728:27
    |
    |
728 |                 self.opts.debugging_opts.wasi_exec_model,


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
734 |         self.opts.cg.split_debuginfo.unwrap_or(self.target.split_debuginfo)
    |                   ^^ private field
error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:739:23
    |
739 |             self.opts.debugging_opts.stack_protector
739 |             self.opts.debugging_opts.stack_protector
    |                       ^^^^^^^^^^^^^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
771 |             || self.opts.cg.force_unwind_tables.unwrap_or(
    |                          ^^ private field

error[E0616]: field `target_triple` of struct `options::Options` is private
    |
    |
783 |             self.opts.target_triple.triple(),


error[E0616]: field `search_paths` of struct `options::Options` is private
    |
    |
784 |             &self.opts.search_paths,


error[E0616]: field `search_paths` of struct `options::Options` is private
    |
    |
793 |             &self.opts.search_paths,


error[E0616]: field `incremental` of struct `options::Options` is private
    |
    |
868 |         self.opts.incremental.as_ref().map(|_| self.incr_comp_session_dir())

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:894:45
    |
    |
894 |         if let Some((ref c, _)) = self.opts.debugging_opts.fuel {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:912:40
    |
    |
912 |         if let Some(ref c) = self.opts.debugging_opts.print_fuel {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:924:19
    |
    |
924 |         self.opts.debugging_opts.threads


error[E0616]: field `cli_forced_codegen_units` of struct `options::Options` is private
    |
    |
930 |         if let Some(n) = self.opts.cli_forced_codegen_units {


error[E0616]: field `incremental` of struct `options::Options` is private
    |
    |
940 |         if self.opts.incremental.is_some() {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:998:19
    |
    |
998 |         self.opts.debugging_opts.teach && self.diagnostic().must_teach(code)


error[E0616]: field `edition` of struct `options::Options` is private
     |
     |
1002 |         self.opts.edition == Edition::Edition2015


error[E0616]: field `edition` of struct `options::Options` is private
     |
     |
1007 |         self.opts.edition >= Edition::Edition2018


error[E0616]: field `edition` of struct `options::Options` is private
     |
     |
1012 |         self.opts.edition >= Edition::Edition2021


error[E0616]: field `edition` of struct `options::Options` is private
     |
1016 |         self.opts.edition
     |                   ^^^^^^^ private field


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1025:35
     |
1025 |         let dbg_opts = &self.opts.debugging_opts;

error[E0616]: field `optimize` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1041:19
     |
     |
1041 |         self.opts.optimize != config::OptLevel::No

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1045:22
     |
     |
1045 |         || self.opts.debugging_opts.sanitizer.intersects(SanitizerSet::ADDRESS | SanitizerSet::MEMORY | SanitizerSet::HWADDRESS)


error[E0616]: field `cg` of struct `options::Options` is private
     |
     |
1049 |         self.opts.cg.link_dead_code.unwrap_or(false)
     |                   ^^ private field
error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1105:33
     |
1105 |     let macro_backtrace = sopts.debugging_opts.macro_backtrace;
1105 |     let macro_backtrace = sopts.debugging_opts.macro_backtrace;
     |                                 ^^^^^^^^^^^^^^ private field

error[E0616]: field `error_format` of struct `options::Options` is private
     |
     |
1106 |     match (sopts.error_format, emitter_dest) {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1113:51
     |
     |
1113 |                 Box::new(emitter.ui_testing(sopts.debugging_opts.ui_testing))

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1120:31
     |
     |
1120 |                         sopts.debugging_opts.teach,

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1121:31
     |
     |
1121 |                         sopts.debugging_opts.terminal_width,

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1134:51
     |
     |
1134 |                 Box::new(emitter.ui_testing(sopts.debugging_opts.ui_testing))

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1143:23
     |
     |
1143 |                 sopts.debugging_opts.terminal_width,

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1146:31
     |
     |
1146 |             .ui_testing(sopts.debugging_opts.ui_testing),

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1155:23
     |
     |
1155 |                 sopts.debugging_opts.terminal_width,

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1158:31
     |
     |
1158 |             .ui_testing(sopts.debugging_opts.ui_testing),


error[E0616]: field `lint_opts` of struct `options::Options` is private
     |
1181 |         .lint_opts
     |          ^^^^^^^^^ private field


error[E0616]: field `lint_cap` of struct `options::Options` is private
     |
     |
1187 |     let cap_lints_allow = sopts.lint_cap.map_or(false, |cap| cap == lint::Allow);


error[E0616]: field `maybe_sysroot` of struct `options::Options` is private
     |
     |
1195 |     let sysroot = match &sopts.maybe_sysroot {


error[E0616]: field `error_format` of struct `options::Options` is private
     |
     |
1203 |         early_error(sopts.error_format, &format!("Error loading host specification: {}", e))


error[E0616]: field `error_format` of struct `options::Options` is private
     |
     |
1206 |         early_warn(sopts.error_format, &warning)

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1210:27
     |
     |
1210 |     let hash_kind = sopts.debugging_opts.src_hash_algorithm.unwrap_or_else(|| {
---

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1404:44
     |
1404 |     if sess.crt_static(None) && !sess.opts.debugging_opts.sanitizer.is_empty() {


error[E0616]: field `cg` of struct `options::Options` is private
     |
     |
1413 |         if sess.opts.cg.lto == config::LtoCli::Unspecified
     |                      ^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
     |
     |
1414 |             || sess.opts.cg.lto == config::LtoCli::No
     |                          ^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
     |
     |
1415 |             || sess.opts.cg.lto == config::LtoCli::Thin
     |                          ^^ private field
error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1421:18
     |
     |
1421 |     if sess.opts.debugging_opts.stack_protector != StackProtector::None {

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    --> compiler/rustc_session/src/session.rs:1425:27
     |
     |
1425 |                 sess.opts.debugging_opts.stack_protector, sess.opts.target_triple
     |                           ^^^^^^^^^^^^^^ private field

error[E0616]: field `target_triple` of struct `options::Options` is private
     |
1425 |                 sess.opts.debugging_opts.stack_protector, sess.opts.target_triple
     |                                                                     ^^^^^^^^^^^^^ private field


error[E0616]: field `crate_name` of struct `options::Options` is private
  --> compiler/rustc_session/src/output.rs:61:36
   |
61 |     if let Some(ref s) = sess.opts.crate_name {


error[E0616]: field `output_types` of struct `options::Options` is private
   --> compiler/rustc_session/src/output.rs:131:49
    |
131 |     if let Some(Some(out_filename)) = sess.opts.output_types.get(&OutputType::Metadata) {


error[E0616]: field `cg` of struct `options::Options` is private
   --> compiler/rustc_session/src/output.rs:135:57
    |
135 |     let libname = format!("{}{}", crate_name, sess.opts.cg.extra_filename);
    |                                                         ^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
   --> compiler/rustc_session/src/output.rs:153:57
    |
153 |     let libname = format!("{}{}", crate_name, sess.opts.cg.extra_filename);
    |                                                         ^^ private field
For more information about this error, try `rustc --explain E0616`.
error: could not compile `rustc_session` due to 121 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
