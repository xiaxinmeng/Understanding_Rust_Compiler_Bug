 LLVM
define internal void @_ZN4main20he4ee6787faa2909bPaaE() unnamed_addr #0 {
entry-block:
  %dropflag_hint_46 = alloca i8
  %d = alloca %Dropper

  /* This is the place the dropflag is set */
  store i8 61, i8* %dropflag_hint_46
  %0 = getelementptr inbounds %Dropper, %Dropper* %d, i32 0, i32 0, !dbg !208
  store i8 -44, i8* %0, !dbg !208
  /* End dropflag set */

  call void @llvm.dbg.declare(metadata %Dropper* %d, metadata !210, metadata !192), !dbg !208

  // drop called
  call void @_ZN7Dropper9drop.256317hcf85f43760592a92E(%Dropper* %d), !dbg !211
  ret void, !dbg !212
}
