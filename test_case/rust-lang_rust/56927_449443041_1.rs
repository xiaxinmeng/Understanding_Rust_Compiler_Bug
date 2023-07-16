
define void @_ZN6test184test17hdf9fe1a59f5e8f01E(%Foo* dereferenceable(64) %x) unnamed_addr #0 {
start:
  %0 = bitcast %Foo* %x to [16 x i32]*
  %1 = getelementptr inbounds [16 x i32], [16 x i32]* %0, i64 0, i64 0
  %2 = getelementptr inbounds [16 x i32], [16 x i32]* %0, i64 0, i64 16
  br label %repeat_loop_header

repeat_loop_header:                               ; preds = %repeat_loop_body, %start
  %3 = phi i32* [ %1, %start ], [ %5, %repeat_loop_body ]
  %4 = icmp ne i32* %3, %2
  br i1 %4, label %repeat_loop_body, label %repeat_loop_next

repeat_loop_body:                                 ; preds = %repeat_loop_header
  store i32 17, i32* %3, align 64
  %5 = getelementptr inbounds i32, i32* %3, i64 1
  br label %repeat_loop_header

repeat_loop_next:                                 ; preds = %repeat_loop_header
  ret void
}
