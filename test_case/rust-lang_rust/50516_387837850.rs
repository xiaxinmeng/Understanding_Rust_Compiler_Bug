llvm
; std::fs::DirBuilder::_create
; Function Attrs: uwtable
define void @_ZN3std2fs10DirBuilder7_create17he9982b501e5a1dccE(%"core::result::Result<(), io::error::Error>"* noalias nocapture sret dereferenceable(16), { i32, i1 }* noalias readonly dereferenceable(8) %self, %"path::Path"* noalias nonnull readonly %path.0, i64 %path.1) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds { i32, i1 }, { i32, i1 }* %self, i32 0, i32 1
  %2 = bitcast i1* %1 to i8*
  %3 = load i8, i8* %2, align 1, !range !0
  %4 = trunc i8 %3 to i1
  br i1 %4, label %bb1, label %bb2

bb1:                                              ; preds = %start
; call std::fs::DirBuilder::create_dir_all
  call void @_ZN3std2fs10DirBuilder14create_dir_all17hcd76f7c5d629ca4bE(%"core::result::Result<(), io::error::Error>"* noalias nocapture sret dereferenceable(16) %0, { i32, i1 }* noalias readonly dereferenceable(8) %self, %"path::Path"* noalias nonnull readonly %path.0, i64 %path.1)
  br label %bb3

bb2:                                              ; preds = %start
  %5 = bitcast { i32, i1 }* %self to i32*
; call std::sys::unix::fs::DirBuilder::mkdir
  call void @_ZN3std3sys4unix2fs10DirBuilder5mkdir17hc7463290099457b9E(%"core::result::Result<(), io::error::Error>"* noalias nocapture sret dereferenceable(16) %0, i32* noalias readonly dereferenceable(4) %5, %"path::Path"* noalias nonnull readonly %path.0, i64 %path.1)
  br label %bb4

bb3:                                              ; preds = %bb1
  br label %bb5

bb4:                                              ; preds = %bb2
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3
  ret void
}
