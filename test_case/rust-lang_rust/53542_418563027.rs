
error: internal compiler error: broken MIR in DefId(0/0:3 ~ foo[317d]::main[0]) (_1 = const 123i32): bad assignment (impl std::cmp::PartialEq<i32> = i32): NoSolution
 --> foo.rs:8:34
  |
8 |     let x: impl PartialEq<i32> = 123_i32;
  |                                  ^^^^^^^
