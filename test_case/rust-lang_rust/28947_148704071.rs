 bash
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --disable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-llvm-version-check --disable-local-rust --llvm-root=/usr
...
$ make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z debug-llvm '
...
time: 20.675; rss: 703MB    translation
  time: 7.623; rss: 402MB   llvm function passes
  time: 154.266; rss: 482MB llvm module passes
  time: 61.107; rss: 516MB  codegen passes
  time: 0.007; rss: 482MB   codegen passes
//so it passed with --disable-debuginfo
//next: changed --disable-debuginfo into --enable-debuginfo to make it fail
...
$ make clean
...
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --enable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-llvm-version-check --disable-local-rust --llvm-root=/usr
...
$ time make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -Z verbose -Z print-link-args -Z debug-llvm '
...
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax
time: 0.391; rss: 58MB  parsing
time: 0.127; rss: 59MB  configuration 1
time: 0.000; rss: 59MB  recursion limit
time: 0.009; rss: 59MB  gated macro checking
time: 0.000; rss: 59MB  crate injection
time: 0.019; rss: 61MB  macro loading
time: 0.000; rss: 61MB  plugin loading
time: 0.000; rss: 61MB  plugin registration
time: 1.831; rss: 94MB  expansion
time: 0.036; rss: 94MB  complete gated feature checking 1
time: 0.322; rss: 94MB  configuration 2
time: 0.166; rss: 94MB  maybe building test harness
time: 0.154; rss: 94MB  prelude injection
time: 0.021; rss: 94MB  checking that all macro invocations are gone
time: 0.037; rss: 94MB  complete gated feature checking 2
time: 0.196; rss: 103MB assigning node ids and indexing ast
time: 0.024; rss: 103MB external crate/lib resolution
time: 0.044; rss: 103MB language item collection
time: 0.696; rss: 148MB resolution
time: 0.033; rss: 143MB lifetime resolution
time: 0.000; rss: 143MB looking for entry point
time: 0.021; rss: 143MB looking for plugin registrar
time: 0.263; rss: 158MB region resolution
time: 0.022; rss: 158MB loop checking
time: 0.024; rss: 158MB static item recursion checking
time: 0.151; rss: 161MB type collecting
time: 0.044; rss: 161MB variance inference
time: 0.455; rss: 202MB coherence checking
time: 23.707; rss: 308MB    type checking
time: 4.946; rss: 319MB const checking
time: 0.198; rss: 320MB privacy checking
time: 0.005; rss: 320MB stability index
time: 0.067; rss: 320MB intrinsic checking
time: 0.051; rss: 320MB effect checking
time: 1.123; rss: 320MB match checking
time: 0.176; rss: 327MB liveness checking
time: 9.925; rss: 327MB borrow checking
time: 5.309; rss: 327MB rvalue checking
time: 0.064; rss: 327MB reachability checking
time: 0.152; rss: 327MB death checking
time: 0.178; rss: 327MB stability checking
time: 0.000; rss: 327MB unused lib feature checking
time: 0.801; rss: 327MB lint checking
time: 0.000; rss: 327MB resolving dependency formats
time: 25.456; rss: 928MB    translation
  time: 9.374; rss: 631MB   llvm function passes
  time: 201.226; rss: 740MB llvm module passes
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

real    9m11.276s
user    8m21.940s
sys 0m40.597s
