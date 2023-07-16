
test.rs:31:5: 31:16 error: unresolved import `rustc::llvm`. There is no `llvm` in `rustc` [E0432]
test.rs:31 use rustc::llvm;
               ^~~~~~~~~~~
test.rs:31:5: 31:16 help: run `rustc --explain E0432` to see a detailed explanation
test.rs:105:12: 105:24 error: the type of this value must be known in this context
test.rs:105         if ee.is_null() {
                       ^~~~~~~~~~~~
test.rs:100:21: 100:25 error: the trait `core::marker::Sized` is not implemented for the type `[std::path::PathBuf]` [E0277]
test.rs:100         let (llmod, deps) = compile_program(program, sysroot.clone())
                                ^~~~
test.rs:100:21: 100:25 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:100:21: 100:25 note: `[std::path::PathBuf]` does not have a constant size known at compile-time
test.rs:100:21: 100:25 note: all local variables must have a statically known size
test.rs:120:21: 120:25 error: the trait `core::marker::Sized` is not implemented for the type `[std::path::PathBuf]` [E0277]
test.rs:120         let (llmod, deps) = compile_program(program, self.sysroot.clone())
                                ^~~~
test.rs:120:21: 120:25 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:120:21: 120:25 note: `[std::path::PathBuf]` does not have a constant size known at compile-time
test.rs:120:21: 120:25 note: all local variables must have a statically known size
test.rs:136:17: 136:29 error: the type of this value must be known in this context
test.rs:136             if !fv.is_null() {
                            ^~~~~~~~~~~~
test.rs:139:26: 139:38 error: the type of this value must be known in this context
test.rs:139                 assert!(!fp.is_null());
                                     ^~~~~~~~~~~~
test.rs:139:17: 139:40 note: in this expansion of assert! (defined in <std macros>)
test.rs:153:17: 153:29 error: the type of this value must be known in this context
test.rs:153             if !gv.is_null() {
                            ^~~~~~~~~~~~
test.rs:156:26: 156:38 error: the type of this value must be known in this context
test.rs:156                 assert!(!gp.is_null());
                                     ^~~~~~~~~~~~
test.rs:156:17: 156:40 note: in this expansion of assert! (defined in <std macros>)
error: aborting due to 7 previous errors
