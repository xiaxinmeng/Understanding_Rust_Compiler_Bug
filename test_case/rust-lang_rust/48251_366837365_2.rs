llvm
define void @bar() unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
; invoke foo::foo2
  invoke void @_ZN3foo4foo217h18e6b2931a4bedc6E()
          to label %bb2 unwind label %funclet_bb1

bb1:                                              ; preds = %funclet_bb1
  call void @llvm.trap()
  unreachable

bb2:                                              ; preds = %start
  ret void

funclet_bb1:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb1
}
