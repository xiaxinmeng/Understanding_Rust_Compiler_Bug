 LLVM
define internal void @_ZN4main20h03656b023385628cPaaE() unnamed_addr #0 {
entry-block:
  %dropflag_hint_46 = alloca i8
  %d = alloca %Dropper
  %0 = alloca i8
  %let = alloca i8

  /* Set dropflag */
  store i8 61, i8* %dropflag_hint_46
  %1 = getelementptr inbounds %Dropper, %Dropper* %d, i32 0, i32 0, !dbg !208
  store i8 -44, i8* %1, !dbg !208
  /* end set dropflag */

  call void @llvm.dbg.declare(metadata %Dropper* %d, metadata !210, metadata !192), !dbg !208
  %2 = bitcast i8* %0 to %Dropper*, !dbg !211
  %3 = bitcast %Dropper* %d to i8*, !dbg !211
  %4 = bitcast %Dropper* %2 to i8*, !dbg !211
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %3, i64 1, i32 1, i1 false)
  %5 = bitcast %Dropper* %d to i8*
  call void @llvm.memset.p0i8.i64(i8* %5, i8 29, i64 1, i32 1, i1 false)
  %6 = load i8, i8* %0, align 1
  store i8 %6, i8* %let, align 1

  // Drop called
  call void @_ZN7Dropper9drop.256317h2845f499b38a318cE(%Dropper* %d), !dbg !213
  ret void, !dbg !214
}
