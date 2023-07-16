 llvm
define internal void @_ZN4main20h231ed58dfe11d80bPcaE() unnamed_addr #0 {
entry-block:
  tail call fastcc void @_ZN8gradient20h2530772adbfd5d63ZbaE(i32 0, i32 -1)
  tail call fastcc void @_ZN8gradient20h2530772adbfd5d63ZbaE(i32 -1, i32 0)
  ret void
}
