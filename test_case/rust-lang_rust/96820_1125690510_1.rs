
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn
define void @foo({ i64, i32 }* nocapture noundef align 8 dereferenceable(16) %rc, i64* nocapture noundef align 8 dereferenceable(8) %r.0, i32* noalias nocapture noundef readonly align 4 dereferenceable(4) %r.1) unnamed_addr #0 {
start:
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn
define void @bar({ i64, i32 }* nocapture noundef align 8 dereferenceable(16) %rc, i64* nocapture noundef align 8 dereferenceable(16) %r) unnamed_addr #0 {
start:
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn
define void @baz({ i64, i32 }* nocapture noundef align 8 dereferenceable(16) %rc, i32* nocapture %r.0, i64* nocapture noundef align 8 dereferenceable(8) %r.1) unnamed_addr #0 {
start:
  ret void
}
