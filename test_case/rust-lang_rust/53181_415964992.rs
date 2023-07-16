llvm
  %_587.sroa.4 = alloca [7 x i8], align 4
...
bb10.i:                                           ; preds = %bb7.i
  ; this is *definitely* unaligned, being of offset 3 into a 4-aligned alloca
  %_587.sroa.4.3.sroa_idx = getelementptr inbounds [7 x i8], [7 x i8]* %_587.sroa.4, i64 0, i64 3, !dbg !247022
  br label %bb13.i, !dbg !247023

...
bb13.i:                                           ; preds = %bb12.i, %bb11.i, %bb10.i
  %_14.sroa.0.0.in.in.i = phi i8* [ %_587.sroa.7.0.sroa_cast427, %bb12.i ], [ %_587.sroa.7.0.sroa_cast427, %bb11.i ], [ %_587.sroa.4.3.sroa_idx, %bb10.i ]
  %_14.sroa.6.0.i = phi i32 [ %_587.sroa.4.0._587.sroa.4.1.qpath_span.sroa.0.0.copyload.i, %bb12.i ], [ %_587.sroa.4.0._587.sroa.4.1.root_span.sroa.0.0.copyload.i, %bb11.i ], [ %5, %bb10.i ], !dbg !247028
  %_14.sroa.0.0.in.i = bitcast i8* %_14.sroa.0.0.in.in.i to i32*, !dbg !247030
  ; but somehow LLVM is emitting a load with align 4 here
  %_14.sroa.0.0.i = load i32, i32* %_14.sroa.0.0.in.i, align 4, !dbg !247028, !alias.scope !247025, !noalias !247026
  %77 = icmp eq i64 %61, 0, !dbg !247031
  br i1 %77, label %_ZN13rustc_resolve8Resolver31lint_if_path_starts_with_module17h8a282e5cd5cfe0bbE.exit, label %bb16.i, !dbg !247034
