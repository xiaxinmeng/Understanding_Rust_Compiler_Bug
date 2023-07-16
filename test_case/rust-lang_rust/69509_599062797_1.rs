
*** IR Dump After Global Value Numbering ***
; Function Attrs: nonlazybind uwtable
define internal fastcc void @"_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..SpecExtend$LT$T$C$I$GT$$GT$9from_iter17h0b0744497f437e22E"(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #5 person
ality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
bb54.i.i.i:
  %vector = alloca %"alloc::vec::Vec<u8>", align 4
  %1 = bitcast %"alloc::vec::Vec<u8>"* %vector to i8*
  call void @llvm.lifetime.start.p0i8(i64 12, i8* nonnull %1)
  %2 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vector, i32 0, i32 0, i32 0
  store i32 1, i32* %2, align 4, !alias.scope !6
  %3 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vector, i32 0, i32 1, i32 1
  store i32 0, i32* %3, align 4, !alias.scope !6
  %4 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %vector, i32 0, i32 3
  store i32 0, i32* %4, align 4, !alias.scope !6
  %5 = tail call i8* @__rust_alloc(i32 100000, i32 1) #9, !noalias !9
  %6 = icmp eq i8* %5, null
  br i1 %6, label %bb58.i.i.i, label %.noexc

bb58.i.i.i:                                       ; preds = %bb54.i.i.i
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h1ee1d5f6ef0fe572E(i32 100000, i32 1), !noalias !9
  unreachable

.noexc:                                           ; preds = %bb54.i.i.i
  %7 = bitcast %"alloc::vec::Vec<u8>"* %vector to i8**
  store i8* %5, i8** %7, align 4, !noalias !9
  store i32 100000, i32* %3, align 4, !noalias !9
  store i8 42, i8* %5, align 1, !noalias !12
  %scevgep = getelementptr i8, i8* %5, i32 1
  call void @llvm.memset.p0i8.i32(i8* align 1 %scevgep, i8 42, i32 99999, i1 false)
  store i32 100000, i32* %4, align 4, !noalias !12
  %8 = bitcast %"alloc::vec::Vec<u8>"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* nonnull align 4 %8, i8* nonnull align 4 %1, i32 12, i1 false)
  call void @llvm.lifetime.end.p0i8(i64 12, i8* nonnull %1)
  ret void
}
