plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
  |
7 | use rustc_middle::ty::GeneratorInteriorTypeCause;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `GeneratorInteriorTypeCause` in `ty`

error[E0609]: no field `generator_interior_types` on type `&TypeckResults<'_>`
    |
    |
206 |                 typeck_results.generator_interior_types.as_ref().skip_binder(),
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `generator_interior_predicates`
Some errors have detailed explanations: E0432, E0609.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `clippy_lints` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
