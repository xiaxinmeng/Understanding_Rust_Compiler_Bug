
warning: broken MIR (_0 = ()): bad assignment (impl std::marker::Copy = ()): Sorts(ExpectedFound { expected: impl std::marker::Copy, found: () })
  --> src/test/ui/span/loan-extend.rs:14:47
   |
14 | fn borrow<'a, T>(_: &'a mut T) -> impl Copy { () }
   |                                               ^^
