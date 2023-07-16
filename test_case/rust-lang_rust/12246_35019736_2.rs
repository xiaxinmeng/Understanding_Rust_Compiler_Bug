 llvm
; after
define void @foo::hac36ebd8661e5cd1aj::v0.0({ void (i8*)**, i8* }*) #4 {
entry-block:
  %1 = getelementptr inbounds { void (i8*)**, i8* }* %0, i32 0, i32 1
  %2 = load i8** %1
  %3 = getelementptr inbounds { void (i8*)**, i8* }* %0, i32 0, i32 0
  %4 = bitcast void (i8*)*** %3 to [1 x i8**]**
  %5 = load [1 x i8**]** %4
  %6 = getelementptr inbounds [1 x i8**]* %5, i32 0, i32 1
  %7 = load i8*** %6
  %8 = bitcast i8** %7 to void (i8*)*
  call void %8(i8* %2)
  ret void
}
