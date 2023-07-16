 bash
$ time LD_LIBRARY_PATH='/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib' x86_64-unknown-linux-gnu/stage0/bin/rustc -O -g --emit llvm-bc src/libsyntax/lib.rs

real    4m45.443s
user    4m41.333s
sys 0m3.127s

$ time opt -O2 -disable-output syntax.bc
!dbg attachment points at wrong subprogram for function
!154953 = distinct !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 38, type: !42660, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: true, function: i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E, templateParams: !1032, variables: !154954)
i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.763, i64 0, metadata !155332, metadata !188522), !dbg !882290
!882290 = !DILocation(line: 113, scope: !155326, inlinedAt: !882291)
!882292 = distinct !DILexicalBlock(scope: !882293, file: !154937, line: 129, column: 56)
!158595 = distinct !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 129, type: !158596, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: true, templateParams: !1032, variables: !158598)
opt: syntax.bc: error: input module is broken!

real    0m12.978s
user    0m12.263s
sys 0m0.657s

$ time opt -O2 -verify-each -disable-output syntax.bc
!dbg attachment points at wrong subprogram for function
!154953 = distinct !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 38, type: !42660, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: true, function: i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E, templateParams: !1032, variables: !154954)
i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.763, i64 0, metadata !155332, metadata !188522), !dbg !882290
!882290 = !DILocation(line: 113, scope: !155326, inlinedAt: !882291)
!882292 = distinct !DILexicalBlock(scope: !882293, file: !154937, line: 129, column: 56)
!158595 = distinct !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 129, type: !158596, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: true, templateParams: !1032, variables: !158598)
opt: syntax.bc: error: input module is broken!

real    0m12.416s
user    0m11.690s
sys 0m0.673s
//yup, fails with -verify-each too  (also tested by adding all other -verify* options, still no change)

//even with the llvm that comes with rust, still fails with -verify-each unless I'm doing it wrong and it uses stuff from the svn llvm that I've installed
$ time LD_LIBRARY_PATH='/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/llvm/Release+Asserts/lib' x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/opt -O2 -verify-each -disable-output syntax.bc 
!dbg attachment points at wrong subprogram for function
!154953 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 38, type: !42660, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: true, function: i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E, templateParams: !1032, variables: !154954)
i64 (%closure.1235*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.68893E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.763, i64 0, metadata !155332, metadata !188522), !dbg !882290
!882290 = !DILocation(line: 113, scope: !155326, inlinedAt: !882291)
!882292 = distinct !DILexicalBlock(scope: !882293, file: !154937, line: 129, column: 56)
!158595 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !154937, file: !154937, line: 129, type: !158596, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: true, templateParams: !1032, variables: !158598)
x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/opt: syntax.bc: error: input module is broken!

real    0m14.177s
user    0m13.450s
sys 0m0.630s

$ opt --version
LLVM (http://llvm.org/):
  LLVM version 3.8.0svn
  Optimized build.
  Default target: x86_64-pc-linux-gnu
  Host CPU: (unknown)

//so what I wanted to try before was  not using -g  to see if the issue disappears and it does:
$ time LD_LIBRARY_PATH='/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage0/lib' x86_64-unknown-linux-gnu/stage0/bin/rustc -O --emit llvm-bc src/libsyntax/lib.rs

real    3m40.865s
user    3m37.717s
sys 0m2.530s

$ time opt -O2 -disable-output syntax.bc
real    1m29.530s
user    1m28.343s
sys 0m0.910s

