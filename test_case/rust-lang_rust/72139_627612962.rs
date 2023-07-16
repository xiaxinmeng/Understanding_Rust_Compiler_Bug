
Summary of Iterator methods
- '*' indicates important methods (as judged by me via profiling, see below)
- 'XXX' indicates possible changes

// No default implementation
* next

// Default implementation that doesn't rely on anything else
* size_hint
- nth
- by_ref

// Default implementation uses next
* try_fold
- cmp_by
- partial_cmp_by
- eq_by
- is_sorted_by

// Default implementation uses fold
* for_each
- count
- last
- fold_first

// Default implementation uses try_fold
* fold (XXX: should be implemented directly?)
* position -> try
* any (could use try_for_each)
* find (could use try_for_each)
- all (could use try_for_each)
- find_map (could use try_for_each)
- try_for_each

// Default implementation uses try_rfold
- rposition -> try_rfold

// Default implementation uses for_each (XXX: should use fold?)
- partition -> for_each
- unzip -> for_each

// Default implementation uses try_for_each (XXX: should use try_fold?)
- try_find

// Other
- partition_in_place -> find + rfind
- is_partitioned -> all + any
- max -> max_by
- min -> min_by
- max_by_key -> map + max_by
- min_by_key -> map + min_by
- max_by -> fold_first + fold
- min_by -> fold_first + fold
- cmp -> cmp_by
- partial_cmp -> partial_cmp_by
- eq -> eq_by
- ne -> eq
- lt -> partial_cmp
- le -> partial_cmp
- gt -> partial_cmp
- ge -> partial_cmp
- is_sorted -> is_sorted_by
- is_sorted_by_key -> map

// Uses an auxiliary adapter (some of those use fold, etc.)
- step_by -> StepBy
- chain -> Chain
- zip -> Zip
- map -> Map
- filter -> Filter
- filter_map -> FilterMap
- enumerate -> Enumerate
- peekable -> Peekable
- skip_while -> SkipWhile
- take_while -> TakeWhile
- map_while -> MapWhile
- skip -> Skip
- take -> Take
- scan -> Scan
- flat_map -> FlatMap
- flatten -> Flatten
- fuse -> Fuse
- inspect -> Inspect
- collect -> FromIterator
- rev -> Rev
- copied -> Copied
- cloned -> Cloned
- cycle -> Cycle
- sum -> Sum
- product -> Product
