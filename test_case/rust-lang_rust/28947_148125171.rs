
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --enable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust
...
$ time ( time make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 RUSTFLAGS="--verbose" && time make check )
cfg: version 1.5.0-dev (294ef5b15 2015-10-14)
cfg: build triple x86_64-unknown-linux-gnu
cfg: host triples x86_64-unknown-linux-gnu
cfg: target triples x86_64-unknown-linux-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: enabling debuginfo (CFG_ENABLE_DEBUGINFO)
cfg: host for x86_64-unknown-linux-gnu is x86_64
cfg: os for x86_64-unknown-linux-gnu is unknown-linux-gnu
cfg: good valgrind for x86_64-unknown-linux-gnu is 1
cfg: using CC=ccache gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax
time: 0.381; rss: 58MB  parsing
time: 0.124; rss: 59MB  configuration 1
...
...
time: 25.515; rss: 929MB    translation
  time: 9.382; rss: 631MB   llvm function passes
  time: 202.128; rss: 738MB llvm module passes
!dbg attachment points at wrong subprogram for function
!156164 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !156148, file: !156148, line: 38, type: !42689, isLocal: true, isDefinition: true, scopeLine: 38, flags: DIFlagPrototyped, isOptimized: true, function: i64 (%closure.1236*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.69460E, templateParams: !1036, variables: !156165)
i64 (%closure.1236*, %"ast::Expr"*)* @_ZN3ext6expand11expand_expr13closure.69460E
  tail call void @llvm.dbg.value(metadata %"ext::expand::IdentRenamer"* %rename_fld.i.857, i64 0, metadata !156543, metadata !189756), !dbg !766304
!766304 = !DILocation(line: 113, scope: !156537, inlinedAt: !766305)
!766306 = distinct !DILexicalBlock(scope: !766307, file: !156148, line: 129, column: 56)
!159806 = !DISubprogram(name: "fnfn", linkageName: "fnfn", scope: !156148, file: !156148, line: 129, type: !159807, isLocal: true, isDefinition: true, scopeLine: 129, flags: DIFlagPrototyped, isOptimized: true, templateParams: !1036, variables: !159809)
LLVM ERROR: Broken function found, compilation aborted!
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.syntax] Error 1
