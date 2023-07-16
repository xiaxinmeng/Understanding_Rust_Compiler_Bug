plain
---- src/builtin.rs - builtin::LOCAL_BINDING_SHADOWS_GLOB_REEXPORT (line 3281) stdout ----
warning: unknown lint: `local_definition_shadows_glob_reexport`
 --> src/builtin.rs:3282:9
  |
2 | #![deny(local_definition_shadows_glob_reexport)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `local_binding_shadows_glob_reexport`
  = note: `#[warn(unknown_lints)]` on by default

warning: 1 warning emitted


Test compiled successfully, but it's marked `compile_fail`.

failures:
    src/builtin.rs - builtin::LOCAL_BINDING_SHADOWS_GLOB_REEXPORT (line 3281)

test result: FAILED. 98 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 1.15s

error: doctest failed, to rerun pass `-p rustc_lint_defs --doc`
