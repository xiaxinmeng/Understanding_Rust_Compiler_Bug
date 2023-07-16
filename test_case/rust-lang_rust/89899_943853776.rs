plain
    Checking core v0.0.0 (/checkout/library/core)
error: unused `std::collections::btree_map::Range` that must be used
   --> library/alloc/benches/btree/map.rs:293:17
    |
293 |                 black_box(map.range(f(i, j)));
    |
    |
    = note: `-D unused-must-use` implied by `-D warnings`
    = note: iterators are lazy and do nothing unless consumed
error: unused `std::collections::btree_map::Iter` that must be used
   --> library/alloc/benches/btree/map.rs:325:13
    |
    |
325 |             black_box(map.iter());
    |
    |
    = note: iterators are lazy and do nothing unless consumed
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:747:5
    |
747 |     map.range((Excluded(2), Excluded(2)));
    |
    |
    = note: `-D unused-must-use` implied by `-D warnings`
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:754:5
    |
754 |     map.range((Included(3), Included(2)));
    |
    |
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:761:5
    |
761 |     map.range((Included(3), Excluded(2)));
    |
    |
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:768:5
    |
768 |     map.range((Excluded(3), Included(2)));
    |
    |
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:775:5
    |
775 |     map.range((Excluded(3), Excluded(2)));
    |
    |
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:786:9
    |
786 |         map.range(Cyclic3::C..=Cyclic3::A);
    |
    |
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
   --> library/alloc/src/collections/btree/map/tests.rs:827:5
    |
827 |     map.range(EvilTwin(5)..=EvilTwin(7));
    |
    |
    = note: iterators are lazy and do nothing unless consumed

error: unused `map::Range` that must be used
    --> library/alloc/src/collections/btree/map/tests.rs:1262:9
     |
1262 |         v.range(t..);
     |
     |
     = note: iterators are lazy and do nothing unless consumed

error: unused `map::RangeMut` that must be used
    --> library/alloc/src/collections/btree/map/tests.rs:1267:9
     |
1267 |         v.range_mut(t..);
     |
     |
     = note: iterators are lazy and do nothing unless consumed

error: unused `set::Iter` that must be used
   --> library/alloc/src/collections/btree/set/tests.rs:616:9
616 |         set.iter();
    |         ^^^^^^^^^^^
    |
    |
    = note: iterators are lazy and do nothing unless consumed
error: build failed
Build completed unsuccessfully in 0:01:39
