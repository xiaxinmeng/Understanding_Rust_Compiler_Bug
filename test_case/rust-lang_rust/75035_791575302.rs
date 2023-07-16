
define void @test() {
start:
  br label %loop 

loop:                                            
  %iv = phi i32 [ 1, %start ], [ %iv.next, %loop ]
  %iv.inc = add nsw i32 %iv, 1
  tail call void @f()
  %cmp1 = icmp slt i32 %iv, 2
  %iv.next = select i1 %cmp1, i32 %iv.inc, i32 2
  %cmp2 = icmp slt i32 %iv.next, 3
  %and = and i1 %cmp1, %cmp2
  br i1 %and, label %loop, label %exit

exit:                                              
  ret void
}

declare void @f() 
