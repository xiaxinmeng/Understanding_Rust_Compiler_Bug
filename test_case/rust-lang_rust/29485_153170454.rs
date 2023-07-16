
declare void @foo()

@g = external global i8

define void @f() {
  store i8 1, i8* @g
  call void @foo() readnone
  store i8 0, i8* @g
  ret void
}
