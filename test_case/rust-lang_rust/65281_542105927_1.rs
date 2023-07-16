rust
   for (k, v) in smallest.items_mut().drain() {
       if second_smallest.items_mut().insert(k, v).is_none() {
           let item_size = estimator.size_estimate(k);
           second_smallest.modify_size_estimate(item_size);
       }
   }
   