llvm
define void @_ZN7example1g17h14264d45cdc1c8ddE(i8* noalias readonly align 1 dereferenceable_or_null(1) %clip) unnamed_addr #1 !dbg !18 {
start:
  %item = alloca %SpecificDisplayItem, align 8
  %_6 = alloca %DI, align 8
  %_2 = call align 1 dereferenceable(1) i8* @"_ZN4core6option15Option$LT$T$GT$6unwrap17hd51eb875dba3900fE"(i8* noalias readonly align 1 dereferenceable_or_null(1) %clip, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc21 to %"std::panic::Location"*)), !dbg !21
  br label %bb1, !dbg !21

bb1:                                              ; preds = %start
  %0 = bitcast %SpecificDisplayItem* %item to i64*, !dbg !22
  store i64 0, i64* %0, align 8, !dbg !22
  %1 = bitcast %DI* %_6 to %SpecificDisplayItem*, !dbg !23
  %2 = bitcast %SpecificDisplayItem* %1 to i8*, !dbg !23
  %3 = bitcast %SpecificDisplayItem* %item to i8*, !dbg !23
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %2, i8* align 8 %3, i64 184, i1 false), !dbg !23
  call void @_ZN7example7do_item17h87ff3f7e8c231d7dE(%DI* noalias readonly align 8 dereferenceable(184) %_6), !dbg !24
  br label %bb2, !dbg !24

bb2:                                              ; preds = %bb1
  ret void, !dbg !25
}
