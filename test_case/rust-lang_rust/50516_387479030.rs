llvm
start:
  %_12 = alloca { i32, i1 }, align 4
  %_10 = alloca i32*, align 8
  %_9 = alloca [1 x { i8*, i8* }], align 8
  %_2 = alloca %"core::fmt::Arguments", align 8
; call std::fs::DirBuilder::new
  %0 = call { i32, i1 } @_ZN3std2fs10DirBuilder3new17h4be627efa14f8660E()
  store { i32, i1 } %0, { i32, i1 }* %_12, align 4
  br label %bb1
