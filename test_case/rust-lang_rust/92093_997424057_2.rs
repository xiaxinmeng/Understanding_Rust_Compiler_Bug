
StorageLive(_1);                 // scope 0 at .\issue-92093\src\main.rs:2:9: 2:12
_1 = [closure@.\issue-92093\src\main.rs:2:15: 2:24]; // scope 0 at .\issue-92093\src\main.rs:2:15:
[compiler_fork.zip](https://github.com/rust-lang/rust/files/7741199/compiler_fork.zip)
[mir_compare.zip](https://github.com/rust-lang/rust/files/7741203/mir_compare.zip)
 2:24
                                 // closure
                                 // + def_id: DefId(0:4 ~ main[8185]::main::{closure#0})
                                 // + substs: [
                                 //     i8,
                                 //     extern "rust-call" fn((&i32,)) -> i32,
                                 //     (),
                                 // ]
