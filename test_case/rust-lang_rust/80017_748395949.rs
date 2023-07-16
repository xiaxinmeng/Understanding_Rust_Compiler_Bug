plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_cc51ffc2-f318-469f-baa3-239999c07fb1
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=sugg-rest-pattern
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_cc51ffc2-f318-469f-baa3-239999c07fb1
GITHUB_REF=refs/pull/80017/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=431677043
GITHUB_RUN_NUMBER=21652
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-q6qqm8p3/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmps333wilhpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
   |
23 | use std::iter;
   |     ^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to previous error

error: could not compile `rustc_typeck`
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_macros" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_fs_util" "-p" "rustc_apfloat" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:46
== clock drift check ==
  local time: Sat Dec 19 01:14:17 UTC 2020
