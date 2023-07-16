 llvm
join:                                             ; preds = %case_body
  call void @_ZN3Foo14glue_drop.146917h02f639dd107d06daE(%struct.Foo* %_2)
  call void @_ZN3Foo14glue_drop.146917h02f639dd107d06daE(%struct.Foo* %_1)
  call void @_ZN3Foo14glue_drop.146917h02f639dd107d06daE(%struct.Foo* %0) ; <-- 3
  %7 = getelementptr inbounds %struct.Foo* %_4, i32 0, i32 1
  store i8 1, i8* %7
  %8 = getelementptr inbounds %struct.Foo* %_4, i32 0, i32 0
  store i64 4, i64* %8
  call void @_ZN3Foo14glue_drop.146917h02f639dd107d06daE(%struct.Foo* %_4)
  ret void
