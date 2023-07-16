plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0050]: method `target_machine_factory` has 3 parameters but the declaration in trait `target_machine_factory` has 4
    |
    |
135 |     fn target_machine_factory(&self, _sess: &Session, _opt_level: OptLevel) -> TargetMachineFactoryFn<Self> {
    |
    |
    = note: `target_machine_factory` from trait: `fn(&Self, &Session, OptLevel, &[std::string::String]) -> Arc<(dyn std::ops::Fn(TargetMachineFactoryConfig) -> Result<<Self as WriteBackendMethods>::TargetMachine, std::string::String> + Send + std::marker::Sync + 'static)>`
For more information about this error, try `rustc --explain E0050`.
error: could not compile `rustc_codegen_gcc` due to previous error
Build completed unsuccessfully in 0:03:27
