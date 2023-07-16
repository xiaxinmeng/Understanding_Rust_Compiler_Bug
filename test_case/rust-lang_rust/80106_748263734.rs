plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_1ed70fe0-d9a5-40fb-a218-7ddcf281212a
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=binder-refactor-part-2
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_1ed70fe0-d9a5-40fb-a218-7ddcf281212a
GITHUB_REF=refs/pull/80106/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=431138358
GITHUB_RUN_NUMBER=21630
---
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-kuz7vgqn/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpjx4n_7l2pip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0599]: no method named `has_escaping_bound_vars` found for reference `&TyS<'_>` in the current scope
   --> compiler/rustc_trait_selection/src/traits/select/confirmation.rs:263:32
    |
263 |         debug_assert!(!self_ty.has_escaping_bound_vars());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&TyS<'_>`
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::rustc_middle::ty::TypeFoldable;`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_trait_selection`
error: could not compile `rustc_trait_selection`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_apfloat" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_serialize" "-p" "rustc_errors" "-p" "rustc_incremental" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_symbol_mangling" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_macros" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:42
== clock drift check ==
  local time: Fri Dec 18 19:00:55 UTC 2020
