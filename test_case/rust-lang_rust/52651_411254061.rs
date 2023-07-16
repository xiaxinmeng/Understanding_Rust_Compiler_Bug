llvm
define void @bar(%A* noalias nocapture sret dereferenceable(16), %A* noalias nocapture readonly dereferenceable(16) %self) unnamed_addr #0 {
 ; ...
}
; ...
define void @foo(%A* noalias nocapture sret dereferenceable(16), %A* noalias nocapture readonly dereferenceable(16)) unnamed_addr #0 {
  tail call void @bar(%A* noalias nocapture sret dereferenceable(16) %0, %A* noalias nocapture readonly dereferenceable(16) %1) #0
  ret void
}
