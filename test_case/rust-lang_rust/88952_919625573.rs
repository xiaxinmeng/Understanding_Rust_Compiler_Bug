plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0433]: failed to resolve: could not find `arm_base` in `super`
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_uclibceabihf.rs:19:38
   |
19 |             unsupported_abis: super::arm_base::unsupported_abis(),
   |                                      ^^^^^^^^ could not find `arm_base` in `super`
    Checking gsgdt v0.1.2
    Checking tracing-serde v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
    Checking rls-data v0.19.1
error[E0560]: struct `TargetOptions` has no field named `unsupported_abis`
  --> compiler/rustc_target/src/spec/armv7_unknown_linux_uclibceabihf.rs:19:13
   |
19 |             unsupported_abis: super::arm_base::unsupported_abis(),
   |             ^^^^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 93 others
Some errors have detailed explanations: E0433, E0560.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `rustc_target` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
