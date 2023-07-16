
  %n = alloca i8
  %n1 = alloca i8
  %n2 = alloca i8
  store i8 -128, i8* %n
  store i8 undef, i8* %n1
  store i8 undef, i8* %n2
  ret void
