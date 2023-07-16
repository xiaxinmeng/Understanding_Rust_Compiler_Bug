
error: internal compiler error: broken MIR in DefId(0/0:3 ~ playground[6d86]::main[0]) (Terminator { source_info: SourceInfo { span: src/main.rs:6:33: 6:51, scope: scope[1] }, kind: _5 = const <std::boxed::Box<T>>::new(move _6) -> [return: bb9, unwind: bb7] }): call dest mismatch (std::boxed::Box<impl std::iter::Iterator> <- std::boxed::Box<std::slice::Iter<'_, i32>>): NoSolution
 --> src/main.rs:6:49
  |
6 |     let b: Box<impl Iterator> = Box::new(a.iter());
  |                                                 ^
