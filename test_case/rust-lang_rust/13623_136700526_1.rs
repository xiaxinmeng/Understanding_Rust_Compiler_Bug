 llvm
define zeroext i1 @_ZN23E...std..cmp..PartialEq2eq20h8d48eb51e9008df6FaaE(%E* noalias nocapture readonly dereferenceable(8), %E* noalias nocapture readonly dereferenceable(8)) unnamed_addr #0 {
entry-block:
  %2 = getelementptr inbounds %E, %E* %0, i64 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds %E, %E* %1, i64 0, i32 0
  %5 = load i32, i32* %4, align 4, !range !0
  %6 = icmp eq i32 %3, %5
  br i1 %6, label %then-block-74-, label %join63

then-block-74-:                                   ; preds = %entry-block
  switch i32 %3, label %match_else [
    i32 0, label %match_case
    i32 1, label %match_case25
    i32 2, label %match_case28
    i32 3, label %match_case31
    i32 4, label %match_case34
    i32 5, label %match_case37
    i32 6, label %match_case40
    i32 7, label %match_case43
  ]

match_else:                                       ; preds = %then-block-74-
  unreachable

match_case:                                       ; preds = %then-block-74-
  %7 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %8 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %9 = load i32, i32* %8, align 4
  %10 = load i32, i32* %7, align 4
  %11 = icmp eq i32 %9, %10
  %12 = zext i1 %11 to i8
  br label %join63

match_case25:                                     ; preds = %then-block-74-
  %13 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %14 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %15 = load i32, i32* %14, align 4
  %16 = load i32, i32* %13, align 4
  %17 = icmp eq i32 %15, %16
  %18 = zext i1 %17 to i8
  br label %join63

match_case28:                                     ; preds = %then-block-74-
  %19 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %20 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %21 = load i32, i32* %20, align 4
  %22 = load i32, i32* %19, align 4
  %23 = icmp eq i32 %21, %22
  %24 = zext i1 %23 to i8
  br label %join63

match_case31:                                     ; preds = %then-block-74-
  %25 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %26 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %27 = load i32, i32* %26, align 4
  %28 = load i32, i32* %25, align 4
  %29 = icmp eq i32 %27, %28
  %30 = zext i1 %29 to i8
  br label %join63

match_case34:                                     ; preds = %then-block-74-
  %31 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %32 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %33 = load i32, i32* %32, align 4
  %34 = load i32, i32* %31, align 4
  %35 = icmp eq i32 %33, %34
  %36 = zext i1 %35 to i8
  br label %join63

match_case37:                                     ; preds = %then-block-74-
  %37 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %38 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %39 = load i32, i32* %38, align 4
  %40 = load i32, i32* %37, align 4
  %41 = icmp eq i32 %39, %40
  %42 = zext i1 %41 to i8
  br label %join63

match_case40:                                     ; preds = %then-block-74-
  %43 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %44 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %45 = load i32, i32* %44, align 4
  %46 = load i32, i32* %43, align 4
  %47 = icmp eq i32 %45, %46
  %48 = zext i1 %47 to i8
  br label %join63

match_case43:                                     ; preds = %then-block-74-
  %49 = getelementptr inbounds %E, %E* %1, i64 0, i32 2, i64 0
  %50 = getelementptr inbounds %E, %E* %0, i64 0, i32 2, i64 0
  %51 = load i32, i32* %50, align 4
  %52 = load i32, i32* %49, align 4
  %53 = icmp eq i32 %51, %52
  %54 = zext i1 %53 to i8
  br label %join63

join63:                                           ; preds = %entry-block, %match_case, %match_case25, %match_case28, %match_case31, %match_case34, %match_case37, %match_case40, %match_case43
  %sret_slot.1 = phi i8 [ %54, %match_case43 ], [ %48, %match_case40 ], [ %42, %match_case37 ], [ %36, %match_case34 ], [ %30, %match_case31 ], [ %24, %match_case28 ], [ %18, %match_case25 ], [ %12, %match_case ], [ 0, %entry-block ]
  %55 = icmp ne i8 %sret_slot.1, 0
  ret i1 %55
}
