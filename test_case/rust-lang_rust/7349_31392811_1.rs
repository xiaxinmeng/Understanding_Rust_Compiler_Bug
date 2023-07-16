 llvm
; Function Attrs: uwtable
define internal i64 @_ZN5outer5inner67h7600d001aed24abe20f7e4097463dde0dedcf59bbd47ee5a6329d5a851388702ap4v0.0E({ i64, %tydesc*, i8*, i8*, i8 }*, i64) unnamed_addr #4 {
"function top level":
  %__arg = alloca i64
  store i64 %1, i64* %__arg
  %2 = load i64* %__arg
  ret i64 %2

"function top level1":                            ; No predecessors!
  %__arg2 = alloca i64
  store i64 %1, i64* %__arg2
  %3 = load i64* %__arg2
  ret i64 %3

"function top level3":                            ; No predecessors!
  %__arg4 = alloca i64
  store i64 %1, i64* %__arg4
  %4 = load i64* %__arg4
  ret i64 %4
}

