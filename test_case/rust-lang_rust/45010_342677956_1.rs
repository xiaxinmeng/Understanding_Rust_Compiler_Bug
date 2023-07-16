llvm
$ rustc +nightly -g line.rs --emit=llvm-ir
$ sed -n '/DIComp/,$p' line.ll
!0 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !1, producer: "clang LLVM (rustc version 1.23.0-nightly (ee2286149 2017-11-07))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2)
!1 = !DIFile(filename: "line.rs", directory: "/tmp")
!2 = !{}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DISubprogram(name: "foo", linkageName: "_ZN4line3fooE", scope: !5, file: !1, line: 2, type: !7, isLocal: false, isDefinition: true, scopeLine: 2, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !2, variables: !2)
!5 = !DINamespace(name: "line", scope: null, file: !6)
!6 = !DIFile(filename: "<unknown>", directory: "")
!7 = !DISubroutineType(types: !8)
!8 = !{!9, !9, !9}
!9 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!10 = !DILocalVariable(name: "x", arg: 1, scope: !4, file: !1, line: 1, type: !9)
!11 = !DIExpression()
!12 = !DILocation(line: 1, scope: !4)
!13 = !DILocalVariable(name: "y", arg: 2, scope: !4, file: !1, line: 1, type: !9)
!14 = !DILocation(line: 6, scope: !4)
