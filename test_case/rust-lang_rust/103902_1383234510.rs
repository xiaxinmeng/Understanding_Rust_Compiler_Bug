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
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:909:13
    |
908 |         let Some((def_id_or_name, output, inputs)) = self.extract_callable_info(
    |                                                           --------------------- arguments to this function are incorrect
909 |             obligation.cause.body_id,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_hir::HirId`, found struct `LocalDefId`
note: associated function defined here
   --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:216:8
    |
216 |     fn extract_callable_info(
