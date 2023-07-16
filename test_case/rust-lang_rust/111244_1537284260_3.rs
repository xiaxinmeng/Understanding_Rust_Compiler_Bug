llvm
; <alloc::collections::vec_deque::VecDeque<T,A> as core::convert::From<alloc::vec::Vec<T,A>>>::from
; Function Attrs: nonlazybind sanitize_address uwtable
define internal void @"_ZN128_$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$T$C$A$GT$$GT$$GT$4from17h1aab7311eff3d202E"(%"alloc::collections::vec_deque::VecDeque<usize>"* sret(%"alloc::collections::vec_deque::VecDeque<usize>") %0, %"alloc::vec::Vec<usize>"* %other) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !530 {
start:
...
  %73 = bitcast %"alloc::vec::Vec<usize>"* %17 to i8*, !dbg !597
  %74 = bitcast %"alloc::vec::Vec<usize>"* %other to i8*, !dbg !597
  %75 = call i8* @__asan_memcpy(i8* %73, i8* %74, i64 24), !dbg !597
; invoke alloc::vec::Vec<T,A>::into_raw_parts_with_alloc
  invoke void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$25into_raw_parts_with_alloc17h212a67106aac7affE"({ i64*, i64, i64, %"alloc::alloc::Global" }* sret({ i64*, i64, i64, %"alloc::alloc::Global" }) %19, %"alloc::vec::Vec<usize>"* %17)
          to label %bb11 unwind label %cleanup, !dbg !597
...
bb18:                                             ; preds = %85, %bb16
  %_54 = load i64, i64* %v2, align 8, !dbg !560
; invoke core::cmp::Ord::max
  %_20 = invoke i64 @_ZN4core3cmp3Ord3max17h98769945f3328bccE(i64 %min_cap, i64 %_54)
          to label %bb19 unwind label %cleanup, !dbg !560

bb19:                                             ; preds = %bb18
  %87 = bitcast i64* %v2 to i8*, !dbg !615
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %87), !dbg !615
; invoke core::num::<impl usize>::next_power_of_two
  %cap = invoke i64 @"_ZN4core3num23_$LT$impl$u20$usize$GT$17next_power_of_two17h45b42a08f606687bE"(i64 %_20)
          to label %bb6 unwind label %cleanup, !dbg !561
bb6:                                              ; preds = %bb19
  store i64 %cap, i64* %cap.dbg.spill, align 8, !dbg !561
  call void @llvm.dbg.declare(metadata i64* %cap.dbg.spill, metadata !540, metadata !DIExpression()), !dbg !616
  %88 = bitcast i64* %_25 to i8*, !dbg !617
  call void @llvm.lifetime.start.p0i8(i64 8, i8* %88), !dbg !617
  store %"alloc::vec::Vec<usize>"* %other, %"alloc::vec::Vec<usize>"** %self.dbg.spill4, align 8, !dbg !617
  call void @llvm.dbg.declare(metadata %"alloc::vec::Vec<usize>"** %self.dbg.spill4, metadata !618, metadata !DIExpression()), !dbg !622
  %self5 = bitcast %"alloc::vec::Vec<usize>"* %other to { i64*, i64 }*, !dbg !622
  store { i64*, i64 }* %self5, { i64*, i64 }** %self.dbg.spill6, align 8, !dbg !622
  call void @llvm.dbg.declare(metadata { i64*, i64 }** %self.dbg.spill6, metadata !623, metadata !DIExpression()), !dbg !627
  br i1 false, label %bb20, label %bb21, !dbg !627

bb21:                                             ; preds = %bb6
  %89 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %self5, i32 0, i32 1, !dbg !627
  %90 = ptrtoint i64* %89 to i64, !dbg !627
  %91 = lshr i64 %90, 3, !dbg !627
  %92 = add i64 %91, 2147450880, !dbg !627
  %93 = inttoptr i64 %92 to i8*, !dbg !627
  %94 = load i8, i8* %93, align 1, !dbg !627
  %95 = icmp ne i8 %94, 0, !dbg !627
  br i1 %95, label %96, label %97, !dbg !627

96:                                               ; preds = %bb21
  call void @__asan_report_load8(i64 %90) #20, !dbg !627
  unreachable, !dbg !627

97:                                               ; preds = %bb21
  %98 = load i64, i64* %89, align 8, !dbg !627
  store i64 %98, i64* %_25, align 8, !dbg !627
  br label %bb22, !dbg !627

bb20:                                             ; preds = %bb6
  store i64 -1, i64* %_25, align 8, !dbg !627
  br label %bb22, !dbg !627

bb22:                                             ; preds = %97, %bb20
  %99 = load i64, i64* %_25, align 8, !dbg !617
  %_24 = icmp ne i64 %99, %cap, !dbg !617
  %100 = bitcast i64* %_25 to i8*, !dbg !629
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %100), !dbg !629
  br i1 %_24, label %bb7, label %bb9, !dbg !617

bb9:                                              ; preds = %bb8, %bb22
  br label %bb10, !dbg !595

bb7:                                              ; preds = %bb22
  %_30 = sub i64 %cap, %len, !dbg !630
; invoke alloc::vec::Vec<T,A>::reserve_exact
  invoke void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$13reserve_exact17h9c2c9f0787252216E"(%"alloc::vec::Vec<usize>"* align 8 %other, i64 %_30)
          to label %bb8 unwind label %cleanup, !dbg !631
...
}
