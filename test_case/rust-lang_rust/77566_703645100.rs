
# Before:
  Lines            Copies         Function name
  -----            ------         -------------
  20188433 (100%)  596578 (100%)  (TOTAL)
    188221 (0.9%)     781 (0.1%)  hashbrown::raw::RawTable<T>::rehash_in_place
    139799 (0.7%)     781 (0.1%)  hashbrown::raw::RawTable<T>::resize
     89815 (0.4%)    1562 (0.3%)  hashbrown::raw::RawTable<T>::rehash_in_place::{{closure}}
     85074 (0.4%)    1289 (0.2%)  hashbrown::raw::RawTable<T>::find
     73140 (0.4%)     636 (0.1%)  hashbrown::raw::RawTable<T>::fallible_with_capacity

# After:
  19598583 (100%)  583746 (100%)  (TOTAL)
     89676 (0.5%)     636 (0.1%)  hashbrown::raw::RawTable<T>::rehash_in_place
     85860 (0.4%)     636 (0.1%)  hashbrown::raw::RawTable<T>::resize
     66576 (0.3%)     552 (0.1%)  hashbrown::raw::RawTable<T>::insert
     63210 (0.3%)     957 (0.2%)  hashbrown::raw::RawTable<T>::find
     58317 (0.3%)     536 (0.1%)  hashbrown::map::HashMap<K,V,S>::insert
     53424 (0.3%)     636 (0.1%)  hashbrown::raw::RawTableInner::fallible_with_capacity
