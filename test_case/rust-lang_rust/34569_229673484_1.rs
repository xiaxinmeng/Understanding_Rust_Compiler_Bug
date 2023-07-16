
!dbg attachment points at wrong subprogram for function
!64 = distinct !DISubprogram(name: "{{closure}}", linkageName: "_ZN4test4main11{{closure}}E", scope: !61, file: !47, line: 4, type: !62, isLocal: true, isDefinition: true, scopeLine: 4, flags: DIFlagPrototyped, isOptimized: false, templateParams: !51, variables: !51)
i1 (i8*, i1)* @"_ZN4test4main28_$u7b$$u7b$closure$u7d$$u7d$17h6ca029a2196c88a1E"
  call void @llvm.dbg.declare(metadata %closure** %__debuginfo_env_ptr, metadata !144, metadata !145), !dbg !146
!146 = !DILocation(line: 4, scope: !60)
!60 = distinct !DISubprogram(name: "{{closure}}", linkageName: "_ZN4test4main11{{closure}}E", scope: !61, file: !47, line: 4, type: !62, isLocal: true, isDefinition: true, scopeLine: 4, flags: DIFlagPrototyped, isOptimized: false, templateParams: !51, variables: !51)
!60 = distinct !DISubprogram(name: "{{closure}}", linkageName: "_ZN4test4main11{{closure}}E", scope: !61, file: !47, line: 4, type: !62, isLocal: true, isDefinition: true, scopeLine: 4, flags: DIFlagPrototyped, isOptimized: false, templateParams: !51, variables: !51)
LLVM ERROR: Broken function found, compilation aborted!
