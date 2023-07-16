 llvm
match_case7:                                      ; preds = %case_body2
  %26 = bitcast %Mode* %21 to { i16, i16 }*
  %27 = getelementptr inbounds { i16, i16 }, { i16, i16 }* %26, i32 0, i32 1
  %28 = load i16, i16* %27, !range !3
  switch i16 %28, label %match_else8 [
    i16 4097, label %match_case9
  ]

...

match_case14:                                     ; preds = %join
  %32 = bitcast %Mode* %29 to { i16, i16 }*
  %33 = getelementptr inbounds { i16, i16 }, { i16, i16 }* %32, i32 0, i32 1
  %34 = load i16, i16* %33, !range !4
  switch i16 %34, label %match_else15 [
    i16 8193, label %match_case16
  ]

...

!3 = !{i16 4096, i16 4112}
!4 = !{i16 8192, i16 8208}
