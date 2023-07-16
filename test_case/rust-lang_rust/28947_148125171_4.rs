
$ make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z debug-llvm -Z          unstable-options -C llvm-args="-verify-debug-info -verify-dom-info -verify-loop-info -verify-regalloc -verify-region-info -verify-scev"'
...
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax
...
time: 25.511; rss: 928MB    translation
  time: 9.280; rss: 631MB   llvm function passes
  time: 200.468; rss: 740MB llvm module passes
!dbg attachment points at wrong subprogram for function
!156127 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !156111, file: !156111, line: 38, type: !42675, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: true, function: i64 (%closure.1236*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.69395E, templateParams: !1032, variables: !156128)
i64 (%closure.1236*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.69395E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.857, i64 0, metadata !156506, metadata !189686), !dbg !765837
!765837 = !DILocation(line: 113, scope: !156500, inlinedAt: !765838)
!765839 = distinct !DILexicalBlock(scope: !765840, file: !156111, line: 129, column: 56)
!159769 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !156111, file: !156111, line: 129, type: !159770, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: true, templateParams: !1032, variables: !159772)
LLVM ERROR: Broken function found, compilation aborted!
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax] Error 1

real    16m51.405s
user    15m38.200s
sys 0m49.577s
