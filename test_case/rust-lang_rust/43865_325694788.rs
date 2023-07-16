rust
    // This method returns the DefId and the BoundRegion corresponding to the given region.
    pub fn is_suitable_region(&self, region: Region<'tcx>) -> Option<FreeRegionInfo> {
        // Find the def-id where this free region is bound. So, for
        // example, if we have something like `fn foo<'a>(x: &'a i32)`
        // where `'a` is early bound, then the binding def-id would be the
        // def-id of `foo`.
        let (binding_scope_def_id, bound_region) = match *region {
            ty::ReFree(ref free_region) => (free_region.scope, free_region.bound_region),
            ty::ReEarlyBound(ref ebr) => {
                (
                    self.tcx.parent_def_id(ebr.def_id),
                    ty::BoundRegion::BrNamed(ebr.def_id, ebr.name),
                )
            }
            _ => return None, // not a free region
        };
        let node_id = self.tcx.hir.as_local_node_id(binding_scope_def_id).unwrap();
        let mut is_impl_item = false;
        match self.tcx.hir.find(node_id) {
            Some(hir_map::NodeItem(..)) |
            Some(hir_map::NodeTraitItem(..)) => {
                // Success -- proceed to return Some below
            }
            Some(hir_map::NodeImplItem(..)) => {
                is_impl_item = self.is_bound_region_in_impl_item(suitable_region_binding_scope);
            }
            _ => return None,
        }
        return Some(FreeRegionInfo {
            def_id: binding_scope_def_id,
            boundregion: bound_region,
            is_impl_item: is_impl_item,
        });
    }
