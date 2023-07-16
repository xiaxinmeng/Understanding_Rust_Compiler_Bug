
; Function Attrs: uwtable
define void @"_ZN4main17_23295c4eb32a43657_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*) #0 {
"function top level":
  %a = alloca %enum.AB
  %__llmatch = alloca i64*
  %_x = alloca i64
  %__trans_ret_slot = alloca {}
  %1 = alloca %str_slice
  %2 = alloca %str_slice
  %3 = getelementptr inbounds %enum.AB* %a, i32 0, i32 0
  store i64 1, i64* %3
  %4 = getelementptr inbounds %enum.AB* %a, i32 0, i32 0
  %5 = load i64* %4, !range !0
  switch i64 %5, label %match_else [
    i64 0, label %match_case
  ]

case_body:                                        ; preds = %match_else, %match_case
  %6 = load i64** %__llmatch
  %7 = load i64* %6
  store i64 %7, i64* %_x
  %8 = getelementptr inbounds %str_slice* %1, i32 0, i32 0
  store i8* getelementptr inbounds ([17 x i8]* @str3379, i32 0, i32 0), i8** %8
  %9 = getelementptr inbounds %str_slice* %1, i32 0, i32 1
  store i64 17, i64* %9
  %10 = getelementptr inbounds %str_slice* %2, i32 0, i32 0
  store i8* getelementptr inbounds ([11 x i8]* @str3380, i32 0, i32 0), i8** %10
  %11 = getelementptr inbounds %str_slice* %2, i32 0, i32 1
  store i64 11, i64* %11
  call void @"_ZN3sys14__extensions__10meth_131479fail_with16_db4c44d01ce411614_0$x2e8$x2dpreE"({}* %__trans_ret_slot, { i64, %tydesc*, i8*, i8*, i8 }* undef, %str_slice* %1, %str_slice* %2, i64 9)
  unreachable

case_body1:                                       ; No predecessors!
  br label %join

match_else:                                       ; preds = %"function top level"
  br label %case_body

match_case:                                       ; preds = %"function top level"
  %12 = bitcast %enum.AB* %a to { i64, i64 }*
  %13 = getelementptr inbounds { i64, i64 }* %12, i32 0, i32 1
  store i64* %13, i64** %__llmatch
  br label %case_body

join:                                             ; preds = %case_body1
  ret void
}
