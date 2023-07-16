 llvm
define dereferenceable(1) i8* @_ZN8get_bool20hd181be98fe457b77eaaE(%"enum.core::option::Option<[\22bool\22]>[#2]"* noalias dereferenceable(2), i1 zeroext) unnamed_addr #0 {
entry-block:
  %2 = getelementptr inbounds %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, i64 0, i32 0
  %3 = load i8* %2, align 1, !range !0, !alias.scope !1
  %4 = icmp eq i8 %3, 1
  br i1 %4, label %"_ZN6option15Option$LT$T$GT$6unwrap21h12513988046761130743E.exit", label %then-block-25-

then-block-25-:                                   ; preds = %entry-block
  %.sroa.3.0.insert.ext = zext i1 %1 to i16
  %.sroa.3.0.insert.shift = shl nuw nsw i16 %.sroa.3.0.insert.ext, 8
  %.sroa.0.0.insert.insert = or i16 %.sroa.3.0.insert.shift, 1
  %5 = bitcast %"enum.core::option::Option<[\22bool\22]>[#2]"* %0 to i16*
  store i16 %.sroa.0.0.insert.insert, i16* %5, align 2
  br label %"_ZN6option15Option$LT$T$GT$6unwrap21h12513988046761130743E.exit"

"_ZN6option15Option$LT$T$GT$6unwrap21h12513988046761130743E.exit": ; preds = %then-block-25-, %entry-block
  %6 = getelementptr inbounds %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, i64 0, i32 2, i64 0
  ret i8* %6
}
