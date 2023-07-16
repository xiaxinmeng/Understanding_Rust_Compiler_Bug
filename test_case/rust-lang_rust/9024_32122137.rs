 llvm
; ModuleID = '<stdin>'

define void @test() {
"function top level":
  br label %loop

loop:                                             ; preds = %body, %"function top level"
  %0 = phi i64 [ 0, %"function top level" ], [ %2, %body ]
  %1 = icmp ugt i64 %0, 2
  br i1 %1, label %fail, label %body

fail:                                             ; preds = %loop
  tail call void @bounds_fail()
  unreachable

body:                                             ; preds = %loop
  %2 = add i64 %0, 1
  %3 = icmp slt i64 %2, 3
  br i1 %3, label %loop, label %out

out:                                              ; preds = %body
  ret void
}

declare void @bounds_fail()
