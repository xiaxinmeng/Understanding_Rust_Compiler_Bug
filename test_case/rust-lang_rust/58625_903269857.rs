
define void @_ZN5test23foo17he3396acf5b2d9080E([1000 x [1000 x i32]]* noalias nocapture sret([1000 x [1000 x i32]]) dereferenceable(4000000) %0) unnamed_addr #0 {
start:
  %1 = getelementptr inbounds [1000 x [1000 x i32]], [1000 x [1000 x i32]]* %0, i64 0, i64 0
  %2 = getelementptr inbounds [1000 x [1000 x i32]], [1000 x [1000 x i32]]* %0, i64 0, i64 1000
  br label %repeat_loop_body2

repeat_loop_body2:                                ; preds = %repeat_loop_body2, %start
  %3 = phi [1000 x i32]* [ %1, %start ], [ %4, %repeat_loop_body2 ]
  %4 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 25
  %.not5.24 = icmp eq [1000 x i32]* %4, %2
  br i1 %.not5.24, label %repeat_loop_next3, label %repeat_loop_body2

repeat_loop_next3:                                ; preds = %repeat_loop_body2
  ret void
}
