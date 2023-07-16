rust
   struct SizeEstimator {
      cache: FxHashMap<MonoItem, usize>,
   }
   impl SizeEstimator {
       fn size_estimate(&mut self, mono_item: &MonoItem) -> usize { ... }
   }
   