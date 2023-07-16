diff
- let key_type = substs.type_at(0);
+ let key_type = cx.tcx.erase_regions(&substs.type_at(0));
