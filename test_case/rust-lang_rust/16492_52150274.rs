 llvm
define internal void @_ZN13DropMe_NoImpl14glue_drop.148117h571786c88b10fe1dE(%"struct.DropMe_NoImpl<[]>"*) unnamed_addr #3 {
entry-block:
  %1 = getelementptr inbounds %"struct.DropMe_NoImpl<[]>"* %0, i32 0, i32 0
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %1)
  %2 = getelementptr inbounds %"struct.DropMe_NoImpl<[]>"* %0, i32 0, i32 1
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %2)
  %3 = getelementptr inbounds %"struct.DropMe_NoImpl<[]>"* %0, i32 0, i32 2
  call void @_ZN7DropMsg14glue_drop.146617h5eadde10e6a29588E(%"struct.DropMsg<[]>"* %3)
  ret void
}
