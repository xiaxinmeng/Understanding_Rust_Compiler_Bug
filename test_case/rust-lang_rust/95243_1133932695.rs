plain
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
    Checking chalk-engine v0.80.0
error: expected one of `,` or `}`, found `.`
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:11:20
10 |     Target {
   |     ------ while parsing this struct
11 |         llvm_target.into(),
11 |         llvm_target.into(),
   |                    ^ expected one of `,` or `}`
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
error[E0063]: missing field `llvm_target` in initializer of `Target`
error[E0063]: missing field `llvm_target` in initializer of `Target`
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:10:5
10 |     Target {
   |     ^^^^^^ missing `llvm_target`

error[E0063]: missing field `llvm_target` in initializer of `spec::Target`
error[E0063]: missing field `llvm_target` in initializer of `spec::Target`
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:10:5
10 |     Target {
   |     ^^^^^^ missing `llvm_target`

For more information about this error, try `rustc --explain E0063`.
