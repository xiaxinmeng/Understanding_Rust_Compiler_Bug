
; <label>:22:                                     ; preds = %24
  store i8 0, i8* %3, align 1
  %23 = load i64*, i64** %4, align 8
  call void @_ZN4test8box_free17hbd5ff355ce749de2E(i64* %23)
  br label %18
