plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0425]: cannot find function `backtrace` in crate `libc`
     |
     |
1329 |         let _depth = libc::backtrace(STACK_TRACE.as_mut_ptr(), 256);
     |                            ^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `MINSIGSTKSZ` in crate `libc`
     |
     |
1343 |         const ALT_STACK_SIZE: usize = libc::MINSIGSTKSZ + 64 * 1024;
     |                                             ^^^^^^^^^^^ not found in `libc`

error[E0412]: cannot find type `stack_t` in crate `libc`
     |
     |
1344 |         let mut alt_stack: libc::stack_t = std::mem::zeroed();
     |                                  ^^^^^^^ not found in `libc`

error[E0425]: cannot find function `sigaltstack` in crate `libc`
     |
     |
1349 |         libc::sigaltstack(&mut alt_stack, std::ptr::null_mut());
     |               ^^^^^^^^^^^ not found in `libc`

error[E0412]: cannot find type `sigaction` in crate `libc`
     |
     |
1351 |         let mut sa: libc::sigaction = std::mem::zeroed();
     |                           ^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `SA_NODEFER` in crate `libc`
     |
     |
1354 |         sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
     |                             ^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `SA_RESETHAND` in crate `libc`
     |
     |
1354 |         sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
     |                                                ^^^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `SA_ONSTACK` in crate `libc`
     |
     |
1354 |         sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
     |                                                                     ^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find function `sigemptyset` in crate `libc`
     |
     |
1355 |         libc::sigemptyset(&mut sa.sa_mask);
     |               ^^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find function `sigaction` in crate `libc`
     |
     |
1356 |         libc::sigaction(libc::SIGSEGV, &sa, std::ptr::null_mut());
     |               ^^^^^^^^^ not found in `libc`

error[E0425]: cannot find function `backtrace` in crate `libc`
     |
     |
1329 |         let _depth = libc::backtrace(STACK_TRACE.as_mut_ptr(), 256);
     |                            ^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `MINSIGSTKSZ` in crate `libc`
     |
     |
1343 |         const ALT_STACK_SIZE: usize = libc::MINSIGSTKSZ + 64 * 1024;
     |                                             ^^^^^^^^^^^ not found in `libc`

error[E0412]: cannot find type `stack_t` in crate `libc`
     |
     |
1344 |         let mut alt_stack: libc::stack_t = std::mem::zeroed();
     |                                  ^^^^^^^ not found in `libc`

error[E0425]: cannot find function `sigaltstack` in crate `libc`
     |
     |
1349 |         libc::sigaltstack(&mut alt_stack, std::ptr::null_mut());
     |               ^^^^^^^^^^^ not found in `libc`

error[E0412]: cannot find type `sigaction` in crate `libc`
     |
     |
1351 |         let mut sa: libc::sigaction = std::mem::zeroed();
     |                           ^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `SA_NODEFER` in crate `libc`
     |
     |
1354 |         sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
     |                             ^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `SA_RESETHAND` in crate `libc`
     |
     |
1354 |         sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
     |                                                ^^^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `SA_ONSTACK` in crate `libc`
     |
     |
1354 |         sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
     |                                                                     ^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find function `sigemptyset` in crate `libc`
     |
     |
1355 |         libc::sigemptyset(&mut sa.sa_mask);
     |               ^^^^^^^^^^^ not found in `libc`

error[E0425]: cannot find function `sigaction` in crate `libc`
     |
     |
1356 |         libc::sigaction(libc::SIGSEGV, &sa, std::ptr::null_mut());
     |               ^^^^^^^^^ not found in `libc`
error: aborting due to 10 previous errors

error: aborting due to 10 previous errors

---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_middle" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_plugin_impl" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_ast" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_ast_pretty" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_ast_passes" "-p" "rustc_builtin_macros" "-p" "rustc_metadata" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:02:27
