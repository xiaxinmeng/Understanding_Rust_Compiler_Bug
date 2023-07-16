plain
    Checking cranelift-frontend v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking cranelift-object v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking cranelift-jit v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0027]: pattern does not mention field `ignored_local_def_ids`
   --> src/driver/jit.rs:105:9
    |
105 |     let EntryFn { local_def_id: main_def_id, is_naked: main_is_naked } =
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `ignored_local_def_ids`
help: include the missing field in the pattern
    |
    |
105 |     let EntryFn { local_def_id: main_def_id, is_naked: main_is_naked, ignored_local_def_ids } =
    |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
105 |     let EntryFn { local_def_id: main_def_id, is_naked: main_is_naked, .. } =

error: aborting due to previous error

For more information about this error, try `rustc --explain E0027`.
