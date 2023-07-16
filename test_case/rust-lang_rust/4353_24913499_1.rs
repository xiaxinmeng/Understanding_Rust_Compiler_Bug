 llvm
define void @_ZN4main18h7c52aaf9b95dc92af4v0.0E({ i64, %tydesc*, i8*, i8*, i8 }*) #4 {
"function top level":
  %1 = alloca i64
  store i64 1, i64* %1
  %2 = load i64* %1
  switch i64 %2, label %match_else [
    i64 0, label %match_case
  ]

case_body:                                        ; preds = %match_case
  br label %join

case_body1:                                       ; preds = %match_else
  br label %join

match_else:                                       ; preds = %"function top level"
  br label %case_body1

match_case:                                       ; preds = %"function top level"
  br label %case_body

join:                                             ; preds = %case_body1, %case_body
  ret void
}
