
error: internal compiler error: broken MIR in DefId(0:6 ~ playground[abce]::main) (_1 = Option::<u32>::Some(const 44_u32)): bad assignment (std::option::Option<impl std::fmt::Debug> = std::option::Option<u32>): NoSolution
 --> src/main.rs:7:33
  |
7 |     let x: Option<impl Debug> = Some(44_u32);
  |                                 ^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:253:27
