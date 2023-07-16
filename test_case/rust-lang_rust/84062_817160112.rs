plain
    Checking cranelift-frontend v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking cranelift-object v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking cranelift-jit v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error: expected `{`, found `=>`
   --> src/driver/jit.rs:108:23
    |
108 |     if !main_is_naked => {
    |     |
    |     this `if` expression has a condition, but no block


warning: unused imports: `c_char`, `c_int`
 --> src/driver/jit.rs:6:20
  |
6 | use std::os::raw::{c_char, c_int};
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0027]: pattern does not mention field `ignored_local_def_ids`
error[E0027]: pattern does not mention field `ignored_local_def_ids`
   --> src/driver/jit.rs:105:9
    |
105 |     let EntryFn { local_def_id: main_def_id, is_naked: main_is_naked } = tcx.entry_fn(LOCAL_CRATE).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `ignored_local_def_ids`
help: include the missing field in the pattern
    |
    |
105 |     let EntryFn { local_def_id: main_def_id, is_naked: main_is_naked, ignored_local_def_ids } = tcx.entry_fn(LOCAL_CRATE).unwrap();
    |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
105 |     let EntryFn { local_def_id: main_def_id, is_naked: main_is_naked, .. } = tcx.entry_fn(LOCAL_CRATE).unwrap();

error[E0027]: pattern does not mention field `ignored_local_def_ids`
  --> src/main_shim.rs:15:14
   |
   |
15 |         Some(EntryFn {local_def_id, is_naked}) => (
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `ignored_local_def_ids`
help: include the missing field in the pattern
   |
   |
15 |         Some(EntryFn {local_def_id, is_naked, ignored_local_def_ids }) => (
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
   |
15 |         Some(EntryFn {local_def_id, is_naked, .. }) => (

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0027`.
