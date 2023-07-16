
; main_rs::device
; Function Attrs: noinline nounwind
define internal i24 @_ZN7main_rs6device17h13d51938dd622c57E(i8 %v) unnamed_addr #2 {
start:
  %0 = alloca %Foo, align 1
  %1 = bitcast %Foo* %0 to i8*
  store i8 %v, i8* %1, align 1
  %2 = getelementptr inbounds %Foo, %Foo* %0, i32 0, i32 1
  store i8 %v, i8* %2, align 1
  %3 = getelementptr inbounds %Foo, %Foo* %0, i32 0, i32 2
  store i8 %v, i8* %3, align 1
  %4 = bitcast %Foo* %0 to i24*
  %5 = load i24, i24* %4, align 1
  ret i24 %5
}
