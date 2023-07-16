plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: unused variable: `features`
   --> src/lib.rs:135:77
    |
135 |     fn target_machine_factory(&self, _sess: &Session, _opt_level: OptLevel, features: &[String]) -> TargetMachineFactoryFn<Self> {
    |                                                                             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_features`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:37
