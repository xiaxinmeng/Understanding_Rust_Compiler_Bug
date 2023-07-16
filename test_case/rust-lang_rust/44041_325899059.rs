llvm
*** IR Dump Before Unroll loops ***
bb3:                                              ; preds = %bb6, %start
  %iter.sroa.0.0 = phi i32* [ getelementptr inbounds ([4 x i32], [4 x i32]* @badopt::table, i64 0, i64 0), %start ], [ %2, %bb6 ]
  %1 = icmp eq i32* %iter.sroa.0.0, getelementptr inbounds ([4 x i32], [4 x i32]* @badopt::table, i64 1, i64 0)
  br i1 %1, label %bb9, label %bb6

bb6:                                              ; preds = %bb3
  %2 = getelementptr inbounds i32, i32* %iter.sroa.0.0, i64 1
  %3 = load i32, i32* %iter.sroa.0.0, align 4
  %4 = icmp eq i32 %3, %0
  br i1 %4, label %bb9, label %bb3
*** IR Dump Before Combine redundant instructions ***
; Function Attrs: nounwind uwtable
define zeroext i1 @badopt::exists_in_table(i32) unnamed_addr #0 {
start:
  br label %bb3

bb3:                                              ; preds = %start
  br i1 false, label %bb9, label %bb6

bb6:                                              ; preds = %bb3
  %1 = icmp eq i32 0, %0
  br i1 %1, label %bb9, label %bb3.1

bb9:                                              ; preds = %bb6.4, %bb3.4, %bb6.3, %bb3.3, %bb6.2, %bb3.2, %bb6.1, %bb3.1, %bb3, %bb6
  %_0.0 = phi i1 [ true, %bb6 ], [ false, %bb3 ], [ false, %bb3.1 ], [ true, %bb6.1 ], [ false, %bb3.2 ], [ true, %bb6.2 ], [ false, %bb3.3 ], [ true, %bb6.3 ], [ false, %bb3.4 ], [ true, %bb6.4 ]
  ret i1 %_0.0

bb3.1:                                            ; preds = %bb6
  br i1 false, label %bb9, label %bb6.1

bb6.1:                                            ; preds = %bb3.1
  %2 = icmp eq i32 0, %0
  br i1 %2, label %bb9, label %bb3.2

bb3.2:                                            ; preds = %bb6.1
  br i1 false, label %bb9, label %bb6.2

bb6.2:                                            ; preds = %bb3.2
  %3 = icmp eq i32 0, %0
  br i1 %3, label %bb9, label %bb3.3

bb3.3:                                            ; preds = %bb6.2
  br i1 false, label %bb9, label %bb6.3

bb6.3:                                            ; preds = %bb3.3
  %4 = icmp eq i32 0, %0
  br i1 %4, label %bb9, label %bb3.4

bb3.4:                                            ; preds = %bb6.3
  br i1 true, label %bb9, label %bb6.4

bb6.4:                                            ; preds = %bb3.4
  br label %bb9
}
