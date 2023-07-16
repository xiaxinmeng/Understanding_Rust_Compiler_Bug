llvm
; std::fs::DirBuilder::_create
; Function Attrs: uwtable
define void @_ZN3std2fs10DirBuilder7_create17he9982b501e5a1dccE(%"core::result::Result<(), io::error::Error>"* noalias nocapture sret dereferenceable(16), { i32, i1 }* noalias nocapture readonly dereferenceable(8) %self, %"path::Path"* noalias nonnull readonly %path.0, i64 %path.1) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds { i32, i1 }, { i32, i1 }* %self, i64 0, i32 1
  %2 = bitcast i1* %1 to i8*
  %3 = load i8, i8* %2, align 1, !range !184
  %4 = icmp eq i8 %3, 0
  br i1 %4, label %bb2, label %bb1

bb1:                                              ; preds = %start
; call std::fs::DirBuilder::create_dir_all
  tail call fastcc void @_ZN3std2fs10DirBuilder14create_dir_all17hcd76f7c5d629ca4bE(%"core::result::Result<(), io::error::Error>"* noalias nocapture nonnull sret dereferenceable(16) %0, { i32, i1 }* noalias nonnull readonly dereferenceable(8) %self, %"path::Path"* noalias nonnull readonly %path.0, i64 %path.1)
  br label %bb5

bb2:                                              ; preds = %start
  %5 = getelementptr inbounds { i32, i1 }, { i32, i1 }* %self, i64 0, i32 0
; call std::sys::unix::fs::DirBuilder::mkdir
  tail call fastcc void @_ZN3std3sys4unix2fs10DirBuilder5mkdir17hc7463290099457b9E(%"core::result::Result<(), io::error::Error>"* noalias nocapture nonnull sret dereferenceable(16) %0, i32* noalias nonnull readonly dereferenceable(4) %5, %"path::Path"* noalias nonnull readonly %path.0, i64 %path.1)
  br label %bb5

bb5:                                              ; preds = %bb2, %bb1
  ret void
}
