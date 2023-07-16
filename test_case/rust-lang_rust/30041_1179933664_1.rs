llvm
define void @bad1(i8* align 1 %x) unnamed_addr #0 {
  %0 = load i8, i8* %x, align 1, !range !2, !noundef !3
  %_2 = trunc i8 %0 to i1
  br i1 %_2, label %bb1, label %bb2

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %1 = load i8, i8* %x, align 1, !range !2, !noundef !3
  %_3 = trunc i8 %1 to i1
  %2 = zext i1 %_3 to i8
  store i8 %2, i8* %x, align 1
  br label %bb2
}

define void @bad2(i8* align 1 %x) unnamed_addr #0 {
  %0 = load i8, i8* %x, align 1, !range !2, !noundef !3
  %_2 = trunc i8 %0 to i1
  br i1 %_2, label %bb1, label %bb2

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  store i8 0, i8* %x, align 1
  store i8 1, i8* %x, align 1
  br label %bb2
}

define void @bad3(i8* align 1 %x) unnamed_addr #0 {
  %0 = load i8, i8* %x, align 1, !range !2, !noundef !3
  %_3 = trunc i8 %0 to i1
  %_2 = xor i1 %_3, true
  br i1 %_2, label %bb1, label %bb2

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %1 = load i8, i8* %x, align 1, !range !2, !noundef !3
  %_4 = trunc i8 %1 to i1
  %2 = zext i1 %_4 to i8
  store i8 %2, i8* %x, align 1
  br label %bb2
}

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
