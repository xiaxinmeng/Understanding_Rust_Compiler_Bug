
bb7.i:                                            ; preds = %bb7.i, %bb22
  %.in = phi double [ %.pre14.i, %bb22 ], [ %239, %bb7.i ]
  %235 = phi double [ %.pre.i, %bb22 ], [ %250, %bb7.i ]
  %iter.sroa.0.013.i = phi i64 [ 0, %bb22 ], [ %237, %bb7.i ]
  %236 = phi <2 x double> [ %231, %bb22 ], [ %252, %bb7.i ]
  %237 = add nuw i64 %iter.sroa.0.013.i, 1
  %238 = getelementptr inbounds double, double* %buf.sroa.0.0.copyload, i64 %iter.sroa.0.013.i
  %239 = load double, double* %238, align 8
  %240 = fmul double %235, %.pre
  %241 = fmul double %239, %.pre86
  %242 = fmul double %.in, %.pre87
  %243 = fadd double %241, %242
  %244 = fmul <2 x double> %236, %233
  %245 = extractelement <2 x double> %244, i32 0
  %246 = fadd double %243, %245
  %247 = extractelement <2 x double> %244, i32 1
  %248 = fsub double %246, %247
  %249 = fsub double %248, %240
  store double %249, double* %238, align 8
  store double %.in, double* %223, align 8
  store double %239, double* %222, align 8
  %250 = extractelement <2 x double> %236, i32 1
  store double %250, double* %225, align 8
  store double %249, double* %224, align 8
  %exitcond.i = icmp eq i64 %237, %buf.sroa.8.0.copyload
  %251 = insertelement <2 x double> undef, double %.in, i32 0
  %252 = insertelement <2 x double> %251, double %249, i32 1
  br i1 %exitcond.i, label %bb17.backedge, label %bb7.i
bb17.backedge:                                    ; preds = %bb7.i
  %234 = add nuw nsw i64 %iter4.sroa.0.080, 1
  %exitcond = icmp eq i64 %234, 100
  br i1 %exitcond, label %bb12.loopexit.loopexit, label %bb22
