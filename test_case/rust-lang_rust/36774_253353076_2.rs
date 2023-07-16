
!dbg attachment points at wrong subprogram for function
!5658 = distinct !DISubprogram(name: "read_f64", linkageName: "_ZN15rustc_serialize4json8{{impl}}8read_f64E", scope: !3127, file: !3126, line: 2109, type: !5659, isLocal: false, isDefinition: true, scopeLine: 2109, flags: DIFlagPrototyped, isOptimized: true, templateParams: !246, variables: !246)
void (%"2.std::result::Result<f64, json::DecoderError>"*, %"json::Decoder"*)* @"_ZN52_$LT$json..Decoder$u20$as$u20$serialize..Decoder$GT$8read_f6417hcbf09878ab2be073E"
  tail call void @llvm.dbg.value(metadata i64 %145, i64 0, metadata !5769, metadata !8453), !dbg !8454
!8454 = !DILocation(line: 1054, scope: !5765)
!5765 = distinct !DISubprogram(name: "abs", linkageName: "_ZN15rustc_serialize3num8{{impl}}3absE", scope: !2633, file: !2632, line: 1054, type: !5766, isLocal: true, isDefinition: true, scopeLine: 1054, flags: DIFlagPrototyped, isOptimized: true, templateParams: !246, variables: !5768)
!5765 = distinct !DISubprogram(name: "abs", linkageName: "_ZN15rustc_serialize3num8{{impl}}3absE", scope: !2633, file: !2632, line: 1054, type: !5766, isLocal: true, isDefinition: true, scopeLine: 1054, flags: DIFlagPrototyped, isOptimized: true, templateParams: !246, variables: !5768)
LLVM ERROR: Broken function found, compilation aborted!
