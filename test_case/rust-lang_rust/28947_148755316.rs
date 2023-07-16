
//3.8.0 svn
$ time opt -O2 -disable-output syntax.bc
real    4m22.447s
user    4m20.920s
sys 0m0.633s

//3.7.0
$ time opt -O2 -disable-output syntax.bc
!dbg attachment points at wrong subprogram for function
!65002 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !64994, file: !64994, line: 38, type: !29584, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: false, function: i64 (%closure.1236*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.69391E, templateParams: !1032, variables: !1032)
i64 (%closure.1236*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.69391E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.686, i64 0, metadata !715374, metadata !76148), !dbg !717137
!717137 = !DILocation(line: 113, scope: !65097, inlinedAt: !717138)
!717139 = distinct !DILexicalBlock(scope: !717140, file: !64994, line: 129, column: 56)
!65968 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !64994, file: !64994, line: 129, type: !65969, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: false, templateParams: !1032, variables: !1032)
LLVM ERROR: Broken function found, compilation aborted!

real    3m43.535s
user    3m42.020s
sys 0m0.753s

