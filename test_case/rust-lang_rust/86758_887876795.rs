llvm
target triple = "i686-unknown-linux-gnu"

define void @my_fn() personality i32 undef {
  invoke void undef()
          to label %6 unwind label %3

1:
  %2 = icmp eq i8 %4, 0
  ret void

3:
  %4 = phi i8 [ 0, %0 ], [ 1, %6 ]
  %5 = landingpad { i8*, i32 }
          cleanup
  br label %1

6:
  invoke fastcc void undef(i32 undef)
          to label %7 unwind label %3

7:
  ret void
}
