llvm
bb63.i:                                           ; preds = %bb62.i
  call void @llvm.lifetime.end.p0i8(i64 48, i8* nonnull %460), !dbg !104817, !noalias !104241
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %353), !dbg !104817, !noalias !104241
  %468 = bitcast %"std::option::Option<std::vec::Vec<u8>>"* %signature.i to {}**, !dbg !104826
  store {}* null, {}** %468, align 8, !dbg !104826, !noalias !104241
  %469 = load i8, i8* %457, align 8, !dbg !104827, !range !3262, !noalias !104241
  %switch.i.i.i = icmp ult i8 %469, 2, !dbg !104827
  br i1 %switch.i.i.i, label %bb107.i, label %bb2.i.i283.i, !dbg !104827

bb2.i.i283.i:                                     ; preds = %bb63.i
  %470 = getelementptr inbounds %"std::io::Error", %"std::io::Error"* %err4.i, i64 0, i32 1, i32 2, i64 7, !dbg !104827
  %471 = bitcast i8* %470 to %"std::io::error::Custom"**, !dbg !104827
  %472 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %471, align 8, !dbg !104669, !noalias !104241, !nonnull !6
  %473 = bitcast %"std::io::error::Custom"* %472 to {}**, !dbg !104663
  %474 = load {}*, {}** %473, align 8, !dbg !104663, !noalias !104430, !nonnull !6
  %475 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %472, i64 0, i32 1, i32 1, !dbg !104663
  %476 = bitcast [3 x i64]** %475 to void ({}*)***, !dbg !104663
  %477 = load void ({}*)**, void ({}*)*** %476, align 8, !dbg !104663, !noalias !104430, !nonnull !6
  %478 = load void ({}*)*, void ({}*)** %477, align 8, !dbg !104663, !invariant.load !6, !noalias !104430, !nonnull !6
  invoke void %478({}* nonnull %474)
          to label %bb3.i.i.i.i.i284.i unwind label %bb44.i, !dbg !104663, !noalias !104430
