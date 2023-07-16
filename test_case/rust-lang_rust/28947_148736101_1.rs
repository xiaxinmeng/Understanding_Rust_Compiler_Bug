
$ time x86_64-unknown-linux-gnu/stage0/bin/rustc -O -g --emit llvm-bc -Csave-temps src/libsyntax/lib.rs

real    4m49.277s
user    4m43.473s
sys 0m4.227s

$ ls -la *.bc
-rw-r--r-- 1 zazdxscf zazdxscf 45058444 Oct 16 16:33 syntax.0.bc
-rw-r--r-- 1 zazdxscf zazdxscf 30834276 Oct 16 16:30 syntax.0.no-opt.bc
-rw-r--r-- 1 zazdxscf zazdxscf 45058444 Oct 16 16:33 syntax.bc
-rw-r--r-- 1 zazdxscf zazdxscf  3356804 Oct 16 16:33 syntax.metadata.bc

$ time opt -O2 -disable-output syntax.bc
!dbg attachment points at wrong subprogram for function
!154953 = distinct !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 38, type: !42660, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: true, function: i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E, templateParams: !1032, variables: !154954)
i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.763, i64 0, metadata !155332, metadata !188522), !dbg !882290
!882290 = !DILocation(line: 113, scope: !155326, inlinedAt: !882291)
!882292 = distinct !DILexicalBlock(scope: !882293, file: !154937, line: 129, column: 56)
!158595 = distinct !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 129, type: !158596, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: true, templateParams: !1032, variables: !158598)
opt: syntax.bc: error: input module is broken!

real    0m13.200s
user    0m12.507s
sys 0m0.627s
//the above also fails with -verify-each also, for obvious reasons as mentioned above by dotdash

$ time opt -O2 -disable-output syntax.0.no-opt.bc

real    5m15.866s
user    5m13.743s
sys 0m0.807s
//yup works without -verify-each
