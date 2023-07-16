none
error: internal compiler error: broken MIR in DefId(0/0:14 ~ bar[7a72]::main[0]) (NoSolution): could not prove Binder(TraitPredicate(<impl Trait<<u32 as std::ops::Add>::Output> as Trait<u32>>))

error: internal compiler error: broken MIR in DefId(0/0:14 ~ bar[7a72]::main[0]) (Terminator { source_info: SourceInfo { span: src/lib.rs:47:5: 47:26, scope: scope[0] }, kind: _1 = const <Either<L, R>>::converge(move _2) -> [return: bb3, unwind: bb4] }): call dest mismatch (<u32 as std::ops::Add>::Output <- u32): NoSolution
