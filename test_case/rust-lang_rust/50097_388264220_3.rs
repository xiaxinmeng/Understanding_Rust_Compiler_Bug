llvm-ir
cleanup.body.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i
  %25 = landingpad { i8*, i32 }
          cleanup
  %26 = bitcast %"std::io::error::Custom"* %7 to i8**
  %27 = load i8*, i8** %26, align 8, !nonnull !6
  %28 = bitcast {}** %10 to i8**
  %29 = load i8*, i8** %28, align 8, !nonnull !6
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17hdda4f80f00ae9ce2E(i8* nonnull %27, i8* noalias nonnull readonly %29) #11
  %30 = bitcast i32* %5 to i64**
  %31 = load i64*, i64** %30, align 8, !nonnull !6
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17hf06ac9a184e5da38E(i64* nonnull %31) #11
  resume { i8*, i32 } %25
