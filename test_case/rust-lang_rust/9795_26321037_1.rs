
        let tps = if_ok!(self.tps(as_.tps, bs.tps));
        let self_ty = if_ok!(self.self_tys(as_.self_ty, bs.self_ty));
        let regions = if_ok!(relate_region_params(self,
                                                  item_def_id,
                                                  &as_.regions,
                                                  &bs.regions));
        Ok(substs { regions: regions,
                    self_ty: self_ty,
                    tps: tps.clone() })
