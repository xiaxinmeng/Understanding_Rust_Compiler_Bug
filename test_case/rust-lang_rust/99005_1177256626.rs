plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
error[E0433]: failed to resolve: could not find `riscv_base` in `super`
  --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:18:38
   |
18 |             unsupported_abis: super::riscv_base::unsupported_abis(),
   |                                      ^^^^^^^^^^ could not find `riscv_base` in `super`
error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:6:21
  |
  |
6 |     base.features = "+m,+a,+f,+d,+c".to_string();
  |     |
  |     expected due to the type of this binding
  |
  = note: expected enum `Cow<'static, str>`
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`

error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:8:16
  |
8 |     base.cpu = "generic-rv64".to_string();
  |     |
  |     expected due to the type of this binding
  |
  = note: expected enum `Cow<'static, str>`
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`

error[E0308]: mismatched types
 --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:9:21
  |
9 |     base.features = "+m,+a,+f,+d,+c".to_string();
  |     |
  |     expected due to the type of this binding
  |
  = note: expected enum `Cow<'static, str>`
  = note: expected enum `Cow<'static, str>`
           found struct `std::string::String`

error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:10:25
   |
10 |     base.llvm_abiname = "lp64d".to_string();
   |     |
   |     expected due to the type of this binding
   |
   = note: expected enum `Cow<'static, str>`
---

error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:15:22
   |
15 |         data_layout: "e-m:e-p:64:64-i64:64-i128:128-n64-S128".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:16:15
   |
16 |         arch: "riscv64".to_string(),
   |
   = note: expected enum `Cow<'static, str>`
            found struct `std::string::String`


error[E0560]: struct `TargetOptions` has no field named `unsupported_abis`
  --> compiler/rustc_target/src/spec/riscv64_linux_android.rs:18:13
   |
18 |             unsupported_abis: super::riscv_base::unsupported_abis(),
   |             ^^^^^^^^^^^^^^^^ `TargetOptions` does not have this field
   |
   = note: available fields are: `is_builtin`, `endian`, `c_int_width`, `os`, `env` ... and 93 others
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
Some errors have detailed explanations: E0308, E0433, E0560.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_target` due to 9 previous errors
