
"BB(0)":                                          ; preds = %entry-block
  %2 = load i32, i32* %arg0, align 4
  store i32 %2, i32* %x, align 4
  %3 = load i32, i32* %arg1, align 4
  store i32 %3, i32* %y, align 4
  %4 = load i32, i32* %x
  %5 = load i32, i32* %y
  %6 = add i32 %4, %5
  br label %"BB(1)"
