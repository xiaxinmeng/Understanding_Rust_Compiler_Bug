plain
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=stack_trace
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_35fb106c-de07-489a-84e6-cc62b15189b5
GITHUB_REF=refs/pull/80182/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=432161486
GITHUB_RUN_NUMBER=21664
---
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-tl6z4fex/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpj8l2jodwpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error: function is never used: `print_stack_trace_on_error_signal`
     |
     |
1326 | fn print_stack_trace_on_error_signal() {
     |
     |
     = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_driver`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_index" "-p" "rustc_lexer" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_parse" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_hir_pretty" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_feature" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:01
== clock drift check ==
  local time: Sat Dec 19 08:59:21 UTC 2020
