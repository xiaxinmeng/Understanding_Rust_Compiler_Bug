Rust
fn opt_normalize_projection_type<'a, 'b, 'gcx, 'tcx>( //
// ...
            let result =             // ...
                Normalized {
                    value: normalized_ty,
                    obligations,
                } // ...
            infcx.projection_cache.borrow_mut()
                                  .complete(projection_ty, &result, cacheable);
            Some(result)
// ...
    fn complete(&mut self,
                key: ty::ProjectionTy<'tcx>,
                value: &NormalizedTy<'tcx>,
                cacheable: bool) {
        let fresh_key = if cacheable {
            debug!("ProjectionCacheEntry::complete: adding cache entry: key={:?}, value={:?}",
                   key, value);
            // <-- WHERE DID value.obligations JUST DISAPPEAR TO?
            self.map.insert(key, ProjectionCacheEntry::NormalizedTy(value.value))
        } else {
            debug!("ProjectionCacheEntry::complete: cannot cache: key={:?}, value={:?}",
                   key, value);
            !self.map.remove(key)
        };

        assert!(!fresh_key, "never started projecting `{:?}`", key);
    }
