 llvm
; Function Attrs: uwtable
define dereferenceable(1) i8* @_ZN8get_bool20h3a2efbe887530bffeaaE(%"enum.core::option::Option<[\22bool\22]>[#2]"* noalias dereferenceable(2)) unnamed_addr #0 {
entry-block:
  %1 = getelementptr inbounds %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, i64 0, i32 0
  %2 = load i8* %1, align 1, !range !0, !alias.scope !1
  %3 = icmp eq i8 %2, 1
  br i1 %3, label %next-block, label %then-block-21-

then-block-21-:                                   ; preds = %entry-block
  store %"enum.core::option::Option<[\22bool\22]>[#2]" { i8 1, [0 x i8] undef, [1 x i8] c"\01" }, %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, align 1
  %.pre = load i8* %1, align 1, !alias.scope !6
  %phitmp = icmp eq i8 %.pre, 1
  br label %next-block

next-block:                                       ; preds = %entry-block, %then-block-21-
  %4 = phi i1 [ true, %entry-block ], [ %phitmp, %then-block-21- ]
  %5 = getelementptr inbounds %"enum.core::option::Option<[\22bool\22]>[#2]"* %0, i64 0, i32 2, i64 0
  %sret_slot.0.i = select i1 %4, i8* %5, i8* null
  %6 = icmp eq i8* %sret_slot.0.i, null
  br i1 %6, label %match_else.i, label %"_ZN6option15Option$LT$T$GT$6unwrap20h3709860458348752072E.exit"

match_else.i:                                     ; preds = %next-block
  tail call void @_ZN9panicking5panic20h53242798fe6f4f1asdlE({ %str_slice, %str_slice, i64 }* noalias readonly dereferenceable(40) @"_ZN6option15Option$LT$T$GT$6unwrap14_MSG_FILE_LINE20hb2632d1dbc781dacwpoE")
  unreachable

"_ZN6option15Option$LT$T$GT$6unwrap20h3709860458348752072E.exit": ; preds = %next-block
  ret i8* %sret_slot.0.i
}
