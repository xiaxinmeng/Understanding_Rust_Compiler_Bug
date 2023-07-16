


bb7.i:                                            ; preds = %bb7.i, %bb22
  %.in87 = phi double [ %.pre14.i, %bb22 ], [ %236, %bb7.i ]
  %232 = phi double [ %.pre.i, %bb22 ], [ %249, %bb7.i ]
  %iter.sroa.0.013.i = phi i64 [ 0, %bb22 ], [ %234, %bb7.i ]
  %233 = phi <2 x double> [ %226, %bb22 ], [ %248, %bb7.i ]
  %234 = add nuw i64 %iter.sroa.0.013.i, 1
  %235 = getelementptr inbounds double, double* %212, i64 %iter.sroa.0.013.i
  %236 = load double, double* %235, align 8
  %237 = fmul double %232, %227
  %238 = fmul double %236, %228
  %239 = fmul double %.in87, %229
  %240 = fadd double %238, %239
  %241 = fmul <2 x double> %233, %231
  %242 = extractelement <2 x double> %241, i32 0
  %243 = fadd double %240, %242
  %244 = extractelement <2 x double> %241, i32 1
  %245 = fsub double %243, %244
  %246 = fsub double %245, %237
  store double %246, double* %235, align 8
  %exitcond.i = icmp eq i64 %234, %local_len.sroa.4.0.lcssa26.i.i
  %247 = insertelement <2 x double> undef, double %.in87, i32 0
  %248 = insertelement <2 x double> %247, double %246, i32 1
  %249 = extractelement <2 x double> %233, i32 1
  br i1 %exitcond.i, label %bb17.loopexit, label %bb7.i
bb17.loopexit:                                    ; preds = %bb7.i
  store double %236, double* %217, align 8
  %216 = add nuw nsw i64 %iter4.sroa.0.080, 1
  store double %.in87, double* %218, align 8
  store double %246, double* %219, align 8
  store double %249, double* %220, align 8
  %exitcond = icmp eq i64 %216, 100
  br i1 %exitcond, label %bb12.loopexit.loopexit, label %bb22
