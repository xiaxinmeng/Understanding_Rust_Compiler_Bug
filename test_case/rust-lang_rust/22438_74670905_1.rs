 llvm
define void @_Z3foov() #0 {
  %x = alloca [1000 x %struct.X], align 16
  %1 = getelementptr inbounds [1000 x %struct.X]* %x, i32 0, i32 0
  %2 = getelementptr inbounds %struct.X* %1, i64 1000
  br label %3

; <label>:3                                       ; preds = %3, %0
  %4 = phi %struct.X* [ %1, %0 ], [ %5, %3 ]
  call void @_ZN1XC2Ev(%struct.X* %4)
  %5 = getelementptr inbounds %struct.X* %4, i64 1
  %6 = icmp eq %struct.X* %5, %2
  br i1 %6, label %7, label %3

; <label>:7                                       ; preds = %3
  ret void
}
