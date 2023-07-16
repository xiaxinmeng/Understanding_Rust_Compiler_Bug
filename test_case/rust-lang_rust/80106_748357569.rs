plain
Collecting six>=1.5 (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.12.197->awscli)
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-orpsf3ft/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmppxhro4ixpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
[RUSTC-TIMING] rustc_save_analysis test:false 1.297
[RUSTC-TIMING] rustc_incremental test:false 1.511
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
 Documenting rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
thread 'rustc' panicked at 'assertion failed: !self_ty.has_escaping_bound_vars()', compiler/rustc_trait_selection/src/traits/select/confirmation.rs:263:9

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'
error: Unrecognized option: 'crate-version'

error: could not document `rustc_metadata`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_metadata compiler/rustc_metadata/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-e2e5f95c94b9ebe5.rmeta --extern memmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap-655c7a611c44abfb.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-7d8cf70f368f78a6.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-e9be6cb5925fb9a6.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a42cc08968175487.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-6ebdab326bc0f697.rmeta --extern rustc_expand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_expand-63ac5e2c8532b5ec.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-197248a062a3bda0.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-a8d75fa202cc88c6.rmeta --extern rustc_hir_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir_pretty-949b2f67ab30ae6e.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-0ffc706890ac3289.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-80027740aa89c402.so --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-196ec2a62a9bf4e4.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-9542c872d89f7c93.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-51b48621a0a0480f.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-11509a0f2f7ec719.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-27d5c8b6ce7a9912.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4ff2fc836b87e217.rmeta --extern snap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsnap-160a126f92fdbe47.rmeta --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-e43bdbdfe3c1fc3d.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-ff5b6f9179259ca1.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.50.0-nightly
  (5552e6dba
  2020-12-18)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] rustc_metadata test:false 4.507
[RUSTC-TIMING] rustc_infer test:false 6.609
[RUSTC-TIMING] rustc_codegen_ssa test:false 5.785
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_target" "-p" "rustc_lexer" "-p" "rustc_metadata" "-p" "rustc_builtin_macros" "-p" "rustc_macros" "-p" "coverage_test_macros" "-p" "rustc_plugin_impl" "-p" "rustc_lint_defs" "-p" "rustc_infer" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_ast" "-p" "rustc_type_ir" "-p" "rustc_typeck" "-p" "rustc_ast_lowering" "-p" "rustc_parse_format" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_llvm" "-p" "rustc_error_codes" "-p" "rustc_middle" "-p" "rustc_index" "-p" "rustc_arena" "-p" "rustc_driver" "-p" "rustc_span" "-p" "rustc_ast_pretty" "-p" "rustc_query_system" "-p" "rustc_ast_passes" "-p" "rustc_session" "-p" "rustc_ty_utils" "-p" "rustc_graphviz" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "-p" "rustc_parse" "-p" "rustc_interface" "-p" "rustc_data_structures" "-p" "rustc_trait_selection" "-p" "rustc_incremental" "-p" "rustc_errors" "-p" "rustc_expand" "-p" "rustc_apfloat" "-p" "rustc_codegen_llvm" "-p" "rustc_attr" "-p" "rustc_mir" "-p" "rustc_traits" "-p" "rustc_fs_util" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_lint" "-p" "rustc_feature"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:44:34
