plain
    Checking chalk-engine v0.76.0
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
error[E0560]: struct `Target` has no field named `supported_sanitizers`
  --> compiler/rustc_target/src/spec/i686_linux_android.rs:24:9
24 |         supported_sanitizers: SanitizerSet::ADDRESS,
   |         ^^^^^^^^^^^^^^^^^^^^ `Target` does not have this field
   |
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`

error[E0560]: struct `Target` has no field named `supported_sanitizers`
  --> compiler/rustc_target/src/spec/x86_64_linux_android.rs:19:9
19 |         supported_sanitizers: SanitizerSet::ADDRESS,
   |         ^^^^^^^^^^^^^^^^^^^^ `Target` does not have this field
   |
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`

For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustc_target` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error[E0560]: struct `spec::Target` has no field named `supported_sanitizers`
  --> compiler/rustc_target/src/spec/i686_linux_android.rs:24:9
24 |         supported_sanitizers: SanitizerSet::ADDRESS,
   |         ^^^^^^^^^^^^^^^^^^^^ `spec::Target` does not have this field
   |
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`

error[E0560]: struct `spec::Target` has no field named `supported_sanitizers`
  --> compiler/rustc_target/src/spec/x86_64_linux_android.rs:19:9
19 |         supported_sanitizers: SanitizerSet::ADDRESS,
   |         ^^^^^^^^^^^^^^^^^^^^ `spec::Target` does not have this field
   |
   = note: available fields are: `llvm_target`, `pointer_width`, `arch`, `data_layout`, `options`
