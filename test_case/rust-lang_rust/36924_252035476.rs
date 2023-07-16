 llvm
target datalayout = "e-m:w-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

declare void @throw()

define void @bad_fn(i64 %val) personality i32 (...)* @__CxxFrameHandler3 {
entry:
  invoke void @throw()
          to label %unreachable unwind label %cleanup1

unreachable:
  unreachable

cleanup1:
  %cleanuppad1 = cleanuppad within none []
  switch i64 %val, label %cleanupdone2 [
    i64 0, label %cleanupdone1
    i64 1, label %cleanupdone1
    i64 6, label %cleanupdone1
  ]

cleanupdone1:
  cleanupret from %cleanuppad1 unwind label %cleanup2

cleanupdone2:
  cleanupret from %cleanuppad1 unwind label %cleanup2

cleanup2:
  %phi = phi i1 [ true, %cleanupdone1 ], [ true, %cleanupdone2 ]
  %cleanuppad2 = cleanuppad within none []
  call void @throw() [ "funclet"(token %cleanuppad2) ]
  unreachable
}

declare i32 @__CxxFrameHandler3(...)
