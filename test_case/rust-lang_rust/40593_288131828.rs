
...
  %128 = and i32 %_8.sroa.0.0.copyload.i.i, 8, !dbg !1314345
  %129 = icmp ne i32 %128, 0, !dbg !1314336
  %130 = and i64 %accum.sroa.0.0.i.i.lcssa, -33, !dbg !1314349
  %.accum.sroa.0.0.i.i = select i1 %129, i64 %130, i64 %accum.sroa.0.0.i.i.lcssa, !dbg !1314350
...
  %134 = call zeroext i1 @_ZN5rustc2ty6AdtDef8has_dtor17hfb2c4f039e32f892E(%"ty::AdtDef"* noalias nonnull readonly dereferenceable(32) %98, %"ty::context::TyCtxt"* noalias nocapture nonnull dereferenceable(8) %arg16), !dbg !1314329
  %135 = or i64 %.accum.sroa.0.0.i.i, 32, !dbg !1314354
  %res.sroa.0.1 = select i1 %134, i64 %135, i64 %.accum.sroa.0.0.i.i, !dbg !1314329
