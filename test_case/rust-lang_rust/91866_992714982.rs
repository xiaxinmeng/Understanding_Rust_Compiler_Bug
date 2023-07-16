plain
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking chalk-solve v0.55.0
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0616]: field `incremental` of struct `options::Options` is private
    |
759 |         self.incremental.is_some()
    |              ^^^^^^^^^^^ private field


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
760 |             || self.debugging_opts.dump_dep_graph


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
761 |             || self.debugging_opts.query_dep_graph


error[E0616]: field `remap_path_prefix` of struct `options::Options` is private
    |
    |
765 |         FilePathMapping::new(self.remap_path_prefix.clone())


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
770 |         !self.debugging_opts.parse_only && // The file is just being parsed


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
771 |             !self.debugging_opts.ls // The file is just being queried


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
776 |         match self.debugging_opts.share_generics {
    |                    ^^^^^^^^^^^^^^ private field


error[E0616]: field `optimize` of struct `options::Options` is private
    |
778 |             None => match self.optimize {
    |                                ^^^^^^^^ private field


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
914 |     for s in sess.opts.debugging_opts.sanitizer {

error[E0616]: field `debug_assertions` of struct `options::Options` is private
   --> compiler/rustc_session/src/config.rs:919:18
    |
    |
919 |     if sess.opts.debug_assertions {
    |                  ^^^^^^^^^^^^^^^^ private field

error[E0616]: field `crate_types` of struct `options::Options` is private
    |
    |
922 |     if sess.opts.crate_types.contains(&CrateType::ProcMacro) {


error[E0616]: field `test` of struct `options::Options` is private
    |
940 |     if sess.opts.test {
    |                  ^^^^ private field


error[E0616]: field `target_triple` of struct `options::Options` is private
    |
    |
953 |         || Target::search(&opts.target_triple, sysroot),


error[E0616]: field `error_format` of struct `options::Options` is private
    |
958 |             opts.error_format,
    |                  ^^^^^^^^^^^^ private field


error[E0616]: field `error_format` of struct `options::Options` is private
    |
    |
967 |         early_warn(opts.error_format, &warning)


error[E0616]: field `error_format` of struct `options::Options` is private
    |
972 |             opts.error_format,
    |                  ^^^^^^^^^^^^ private field


error[E0616]: field `json_future_incompat` of struct `options::Options` is private
    |
    |
283 |         if !self.opts.json_future_incompat {


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
451 |         if self.opts.debugging_opts.print_type_sizes


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
452 |             || self.opts.debugging_opts.query_dep_graph


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
453 |             || self.opts.debugging_opts.dump_mir.is_some()


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
454 |             || self.opts.debugging_opts.unpretty.is_some()


error[E0616]: field `output_types` of struct `options::Options` is private
    |
    |
455 |             || self.opts.output_types.contains_key(&OutputType::Mir)


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
538 |         self.opts.debugging_opts.verbose


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
541 |         self.opts.debugging_opts.time_passes || self.opts.debugging_opts.time


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
541 |         self.opts.debugging_opts.time_passes || self.opts.debugging_opts.time


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
544 |         self.opts.debugging_opts.instrument_mcount


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
547 |         self.opts.debugging_opts.time_llvm_passes


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
550 |         self.opts.debugging_opts.meta_stats


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
553 |         self.opts.debugging_opts.asm_comments


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
556 |         self.opts.debugging_opts.verify_llvm_ir || option_env!("RUSTC_VERIFY_LLVM_IR").is_some()


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
559 |         self.opts.debugging_opts.print_llvm_passes


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
562 |         self.opts.debugging_opts.binary_dep_depinfo


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
593 |         match self.opts.cg.lto {
    |                         ^^ private field

error[E0616]: field `cli_forced_thinlto_off` of struct `options::Options` is private
    |
    |
607 |                 return if self.opts.cli_forced_thinlto_off {


error[E0616]: field `cli_forced_thinlto_off` of struct `options::Options` is private
    |
    |
623 |         if self.opts.cli_forced_thinlto_off {


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
629 |         if let Some(enabled) = self.opts.debugging_opts.thinlto {


error[E0616]: field `optimize` of struct `options::Options` is private
    |
645 |         match self.opts.optimize {
    |                         ^^^^^^^^ private field


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
654 |         self.opts.cg.panic.unwrap_or(self.target.panic_strategy)
    |                   ^^ private field

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
657 |         if let Some(fewer_names) = self.opts.debugging_opts.fewer_names {


error[E0616]: field `output_types` of struct `options::Options` is private
    |
    |
660 |             let more_names = self.opts.output_types.contains_key(&OutputType::LlvmAssembly)


error[E0616]: field `output_types` of struct `options::Options` is private
    |
    |
661 |                 || self.opts.output_types.contains_key(&OutputType::Bitcode)


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
663 |                 || self.opts.debugging_opts.sanitizer.intersects(SanitizerSet::ADDRESS | SanitizerSet::MEMORY);


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
669 |         self.opts.debugging_opts.unstable_options


error[E0616]: field `unstable_features` of struct `options::Options` is private
    |
    |
672 |         self.opts.unstable_features.is_nightly_build()


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
675 |         self.opts.debugging_opts.sanitizer.contains(SanitizerSet::CFI)


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
678 |         self.opts.cg.overflow_checks.unwrap_or(self.opts.debug_assertions)
    |                   ^^ private field
error[E0616]: field `debug_assertions` of struct `options::Options` is private
   --> compiler/rustc_session/src/session.rs:678:58
    |
    |
678 |         self.opts.cg.overflow_checks.unwrap_or(self.opts.debug_assertions)


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
688 |         let requested_features = self.opts.cg.target_feature.split(',');
    |                                            ^^ private field

error[E0616]: field `crate_types` of struct `options::Options` is private
    |
    |
695 |             || crate_type == None && self.opts.crate_types.contains(&CrateType::ProcMacro)


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
707 |         self.opts.cg.relocation_model.unwrap_or(self.target.relocation_model)
    |                   ^^ private field

error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
711 |         self.opts.cg.code_model.or(self.target.code_model)
    |                   ^^ private field

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
715 |         self.opts.debugging_opts.tls_model.unwrap_or(self.target.tls_model)


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
721 |                 self.opts.debugging_opts.wasi_exec_model,


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
727 |         self.opts.cg.split_debuginfo.unwrap_or(self.target.split_debuginfo)
    |                   ^^ private field

error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
732 |             self.opts.debugging_opts.stack_protector
    |                       ^^^^^^^^^^^^^^ private field


error[E0616]: field `cg` of struct `options::Options` is private
    |
    |
764 |             || self.opts.cg.force_unwind_tables.unwrap_or(
    |                          ^^ private field

error[E0616]: field `target_triple` of struct `options::Options` is private
    |
    |
776 |             self.opts.target_triple.triple(),


error[E0616]: field `search_paths` of struct `options::Options` is private
    |
    |
777 |             &self.opts.search_paths,


error[E0616]: field `search_paths` of struct `options::Options` is private
    |
    |
786 |             &self.opts.search_paths,


error[E0616]: field `incremental` of struct `options::Options` is private
    |
    |
861 |         self.opts.incremental.as_ref().map(|_| self.incr_comp_session_dir())


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
887 |         if let Some((ref c, _)) = self.opts.debugging_opts.fuel {


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
905 |         if let Some(ref c) = self.opts.debugging_opts.print_fuel {


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
917 |         self.opts.debugging_opts.threads


error[E0616]: field `cli_forced_codegen_units` of struct `options::Options` is private
    |
    |
923 |         if let Some(n) = self.opts.cli_forced_codegen_units {


error[E0616]: field `incremental` of struct `options::Options` is private
    |
    |
933 |         if self.opts.incremental.is_some() {


error[E0616]: field `debugging_opts` of struct `options::Options` is private
    |
    |
991 |         self.opts.debugging_opts.teach && self.diagnostic().must_teach(code)


error[E0616]: field `edition` of struct `options::Options` is private
    |
    |
995 |         self.opts.edition == Edition::Edition2015


error[E0616]: field `edition` of struct `options::Options` is private
     |
     |
1000 |         self.opts.edition >= Edition::Edition2018


error[E0616]: field `edition` of struct `options::Options` is private
     |
     |
1005 |         self.opts.edition >= Edition::Edition2021


error[E0616]: field `edition` of struct `options::Options` is private
     |
1009 |         self.opts.edition
     |                   ^^^^^^^ private field


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1018 |         let dbg_opts = &self.opts.debugging_opts;


error[E0616]: field `optimize` of struct `options::Options` is private
     |
     |
1034 |         self.opts.optimize != config::OptLevel::No


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1038 |         || self.opts.debugging_opts.sanitizer.intersects(SanitizerSet::ADDRESS | SanitizerSet::MEMORY | SanitizerSet::HWADDRESS)


error[E0616]: field `cg` of struct `options::Options` is private
     |
     |
1042 |         self.opts.cg.link_dead_code.unwrap_or(false)
     |                   ^^ private field

error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
1098 |     let macro_backtrace = sopts.debugging_opts.macro_backtrace;
     |                                 ^^^^^^^^^^^^^^ private field


error[E0616]: field `error_format` of struct `options::Options` is private
     |
     |
1099 |     match (sopts.error_format, emitter_dest) {


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1106 |                 Box::new(emitter.ui_testing(sopts.debugging_opts.ui_testing))


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1113 |                         sopts.debugging_opts.teach,


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1114 |                         sopts.debugging_opts.terminal_width,


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1127 |                 Box::new(emitter.ui_testing(sopts.debugging_opts.ui_testing))


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1136 |                 sopts.debugging_opts.terminal_width,


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1139 |             .ui_testing(sopts.debugging_opts.ui_testing),


error[E0616]: field `debugging_opts` of struct `options::Options` is private
     |
     |
1148 |                 sopts.debugging_opts.terminal_width,
