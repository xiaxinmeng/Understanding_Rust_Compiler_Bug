plain
    Checking cranelift-frontend v0.73.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f)
    Checking cranelift-object v0.73.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f)
    Checking cranelift-jit v0.73.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0433]: failed to resolve: use of undeclared type `EntryFnType`
   --> src/driver/jit.rs:145:9
    |
145 |         EntryFnType::Main => {
    |         ^^^^^^^^^^^ use of undeclared type `EntryFnType`

error[E0433]: failed to resolve: use of undeclared type `EntryFnType`
   --> src/driver/jit.rs:166:9
    |
166 |         EntryFnType::Start => {
    |         ^^^^^^^^^^^ use of undeclared type `EntryFnType`

error[E0425]: cannot find value `CURRENT_MODULE` in this scope
   --> src/driver/jit.rs:158:13
    |
158 |             CURRENT_MODULE.with(|current_module| {


error[E0425]: cannot find value `CURRENT_MODULE` in this scope
   --> src/driver/jit.rs:182:13
    |
182 |             CURRENT_MODULE.with(|current_module| {

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0425, E0433.
