llvm
bb60.i:                                           ; preds = %bb59.i
  call void @llvm.lifetime.end.p0i8(i64 48, i8* nonnull %456), !dbg !99333, !noalias !98802
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %383), !dbg !99333, !noalias !98802
  %464 = bitcast %"core::option::Option<alloc::vec::Vec<u8>>"* %signature.i to {}**, !dbg !99342
  store {}* null, {}** %464, align 8, !dbg !99342, !noalias !98802
  %465 = load i8, i8* %453, align 8, !dbg !99343, !range !15343, !noalias !98802
  %switch.i.i254.i = icmp ult i8 %465, 2, !dbg !99343
  br i1 %switch.i.i254.i, label %bb61.i, label %bb2.i.i.i235, !dbg !99343

bb2.i.i.i235:                                     ; preds = %bb60.i
  %466 = getelementptr inbounds %"std::io::error::Error", %"std::io::error::Error"* %err4.i, i64 0, i32 1, i32 2, i64 7, !dbg !99343
  %467 = bitcast i8* %466 to %"std::io::error::Custom"**, !dbg !99343
  %468 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %467, align 8, !dbg !99346, !noalias !98802, !nonnull !6
  %469 = bitcast %"std::io::error::Custom"* %468 to {}**, !dbg !99348
  %470 = load {}*, {}** %469, align 8, !dbg !99348, !nonnull !6
  %471 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %468, i64 0, i32 1, i32 1, !dbg !99348
  %472 = bitcast [3 x i64]** %471 to void ({}*)***, !dbg !99348
  %473 = load void ({}*)**, void ({}*)*** %472, align 8, !dbg !99348, !nonnull !6
  %474 = load void ({}*)*, void ({}*)** %473, align 8, !dbg !99348, !invariant.load !6, !nonnull !6
  invoke void %474({}* nonnull %470)
          to label %bb3.i.i.i.i.i255.i unwind label %cleanup.i.i.i.i.i.i238, !dbg !99348
