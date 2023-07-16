llvm
bb63.i:                                           ; preds = %bb62.i
  call void @llvm.lifetime.end.p0i8(i64 48, i8* nonnull %413), !dbg !100148, !noalias !99592
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %335), !dbg !100148, !noalias !99592
  %421 = bitcast %"core::option::Option<alloc::vec::Vec<u8>>"* %signature.i to {}**, !dbg !100157
  store {}* null, {}** %421, align 8, !dbg !100157, !noalias !99592
  %422 = load i8, i8* %410, align 8, !dbg !100158, !range !19089, !noalias !99592
  %switch.i.i283.i = icmp ult i8 %422, 2, !dbg !100158
  br i1 %switch.i.i283.i, label %bb64.i, label %bb2.i.i.i, !dbg !100158

bb2.i.i.i:                                        ; preds = %bb63.i
  %423 = getelementptr inbounds %"std::io::error::Error", %"std::io::error::Error"* %err4.i, i64 0, i32 1, i32 2, i64 7, !dbg !100158
  %424 = bitcast i8* %423 to %"std::io::error::Custom"**, !dbg !100158
  %425 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %424, align 8, !dbg !100161, !noalias !99592, !nonnull !6
  %426 = bitcast %"std::io::error::Custom"* %425 to {}**, !dbg !100163
  %427 = load {}*, {}** %426, align 8, !dbg !100163, !noalias !99730, !nonnull !6
  %428 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %425, i64 0, i32 1, i32 1, !dbg !100163
  %429 = bitcast [3 x i64]** %428 to void ({}*)***, !dbg !100163
  %430 = load void ({}*)**, void ({}*)*** %429, align 8, !dbg !100163, !noalias !99730, !nonnull !6
  %431 = load void ({}*)*, void ({}*)** %430, align 8, !dbg !100163, !invariant.load !6, !noalias !99730, !nonnull !6
  invoke void %431({}* nonnull %427)
          to label %bb3.i.i.i.i.i285.i unwind label %cleanup.i.i.i.i.i287.i, !dbg !100163, !noalias !99730
