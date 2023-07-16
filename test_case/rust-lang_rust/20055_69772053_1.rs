
  %18 = load i8** %__arg
  store i64 %16, i64* %__arg1
  %19 = load i64* %__arg1
  store i32 %17, i32* %__arg2
  %20 = load i32* %__arg2
  call void @je_sdallocx(i8* %18, i64 %19, i32 %20)
  ret void
