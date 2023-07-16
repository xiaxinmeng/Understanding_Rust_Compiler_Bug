llvm
; test::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4test4main17h4c216eb98fecdd36E() unnamed_addr #0 {
start:
  %_1 = alloca i8, align 1
  store i8 1, i8* %_1, align 1
  %0 = load i8, i8* %_1, align 1, !range !3    ; %0 = 1
  %1 = sub i8 %0, 0                            ; %1 = 1
  %2 = icmp ule i8 %1, 3                       ; %2 = 1 <= 3 = 1
  %3 = zext i8 %1 to i64                       ; %3 = 1
  %4 = select i1 %2, i64 %3, i64 0             ; %4 = 0
  %5 = icmp eq i64 %4, 1                       ; %5 = 0
  br i1 %5, label %bb1, label %bb2             ; -> bb1

bb1:                                              ; preds = %start
  unreachable

bb2:                                              ; preds = %start
  ret void
}

!3 = !{i8 0, i8 4}
