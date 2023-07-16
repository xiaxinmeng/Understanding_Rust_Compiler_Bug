
; <label>:555:                                    ; preds = %552
  %556 = load <4 x i64>, <4 x i64>* bitcast (<{ [40 x i8] }>* @_ZN3std9panicking12LOCAL_STDERR7__getit5__KEY17h5e656a3d08607c39E.llvm.17348687890923447381 to <4 x i64>*), align 32, !dbg !15680, !alias.scope !15686, !noalias !15691
  store <4 x i64> <i64 1, i64 0, i64 0, i64 undef>, <4 x i64>* bitcast (<{ [40 x i8] }>* @_ZN3std9panicking12LOCAL_STDERR7__getit5__KEY17h5e656a3d08607c39E.llvm.17348687890923447381 to <4 x i64>*), align 32, !dbg !15697, !noalias !15698
  %557 = extractelement <4 x i64> %556, i32 0, !dbg !15699
  %558 = extractelement <4 x i64> %556, i32 2, !dbg !15699
  %559 = extractelement <4 x i64> %556, i32 3, !dbg !15699
  %560 = icmp eq i64 %557, 0, !dbg !15700
  %561 = icmp eq i64 %558, 0, !dbg !15702
  %562 = or i1 %560, %561, !dbg !15700
  br i1 %562, label %573, label %563, !dbg !15700
