plain
    Checking cranelift-frontend v0.93.0
    Checking cranelift-native v0.93.0
    Checking cranelift-object v0.93.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0053]: method `join_codegen` has an incompatible type for trait
    |
    |
226 |     ) -> Result<(CodegenResults, FxHashMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
    |          |
    |          |
    |          expected `IndexMap<WorkProductId, ..., ...>`, found `HashMap<WorkProductId, ..., ...>`
    |          help: change the output type to match the trait: `std::result::Result<(CodegenResults, indexmap::map::IndexMap<WorkProductId, WorkProduct, BuildHasherDefault<FxHasher>>), ErrorGuaranteed>`
    |
    = note: expected signature `fn(&CraneliftCodegenBackend, std::boxed::Box<_>, &Session, &OutputFilenames) -> std::result::Result<(CodegenResults, indexmap::map::IndexMap<WorkProductId, WorkProduct, BuildHasherDefault<FxHasher>>), _>`
               found signature `fn(&CraneliftCodegenBackend, std::boxed::Box<_>, &Session, &OutputFilenames) -> std::result::Result<(CodegenResults, HashMap<WorkProductId, WorkProduct, BuildHasherDefault<FxHasher>>), _>`
For more information about this error, try `rustc --explain E0053`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:01:40
