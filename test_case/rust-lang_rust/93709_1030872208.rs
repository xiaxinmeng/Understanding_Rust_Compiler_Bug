plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0599]: no method named `first_key_value` found for struct `BTreeMap` in the current scope
   --> library/alloc/benches/btree/map.rs:234:27
    |
234 |             black_box(map.first_key_value());


error[E0599]: no method named `last_key_value` found for struct `BTreeMap` in the current scope
   --> library/alloc/benches/btree/map.rs:235:27
    |
235 |             black_box(map.last_key_value());


error[E0599]: no method named `pop_first` found for struct `BTreeMap` in the current scope
   --> library/alloc/benches/btree/map.rs:413:19
    |
413 |         while map.pop_first().is_some() {}
    |                   ^^^^^^^^^ method not found in `BTreeMap<usize, usize>`

error[E0599]: no method named `pop_first` found for struct `BTreeMap` in the current scope
   --> library/alloc/benches/btree/map.rs:484:19
    |
484 |         while map.pop_first().is_some() {}
    |                   ^^^^^^^^^ method not found in `BTreeMap<usize, usize>`

error[E0599]: no method named `pop_first` found for struct `BTreeMap` in the current scope
   --> library/alloc/benches/btree/map.rs:555:19
    |
555 |         while map.pop_first().is_some() {}
    |                   ^^^^^^^^^ method not found in `BTreeMap<usize, [usize; 256]>`

error[E0599]: no method named `pop_first` found for struct `BTreeSet` in the current scope
  --> library/alloc/benches/btree/set.rs:96:19
   |
96 |         while set.pop_first().is_some() {}
   |                   ^^^^^^^^^ method not found in `BTreeSet<usize>`

error[E0599]: no method named `pop_first` found for struct `BTreeSet` in the current scope
   --> library/alloc/benches/btree/set.rs:167:19
    |
167 |         while set.pop_first().is_some() {}
    |                   ^^^^^^^^^ method not found in `BTreeSet<usize>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `alloc` due to 7 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:151:29
    |
151 |         let last_key = *map.last_key_value().unwrap().0;
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:163:29
    |
163 |         let last_key = *map.last_key_value().unwrap().0;
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:210:20
    |
210 |     assert_eq!(map.first_key_value(), Some((&0, &0)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:211:20
    |
211 |     assert_eq!(map.last_key_value(), Some((&(size - 1), &(10 * (size - 1)))));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:212:20
    |
212 |     assert_eq!(map.first_entry().unwrap().key(), &0);
    |                    ^^^^^^^^^^^ method not found in `map::BTreeMap<usize, usize>`
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:213:20
    |
213 |     assert_eq!(map.last_entry().unwrap().key(), &(size - 1));
    |                    ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:258:20
    |
258 |     assert_eq!(map.first_key_value(), None);
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:259:20
259 |     assert_eq!(map.last_key_value(), None);
    |                    ^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `get_key_value`
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:276:20
    |
276 |     assert_eq!(map.first_key_value(), Some((&1, &1)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:277:20
    |
277 |     assert_eq!(map.last_key_value(), Some((&1, &1)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:284:20
    |
284 |     assert_eq!(map.first_key_value(), Some((&1, &2)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:285:20
    |
285 |     assert_eq!(map.last_key_value(), Some((&1, &2)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:296:20
    |
296 |     assert_eq!(map.first_key_value(), Some((&1, &2)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:297:20
    |
297 |     assert_eq!(map.last_key_value(), Some((&2, &4)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:310:20
    |
310 |     assert_eq!(map.first_key_value(), Some((&2, &4)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:311:20
    |
311 |     assert_eq!(map.last_key_value(), Some((&2, &4)));
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:322:20
    |
322 |     assert_eq!(map.first_key_value(), None);
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
   --> library/alloc/src/collections/btree/map/tests.rs:323:20
323 |     assert_eq!(map.last_key_value(), None);
    |                    ^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `get_key_value`
    |
   ::: library/alloc/src/collections/btree/map.rs:166:1
   ::: library/alloc/src/collections/btree/map.rs:166:1
    |
166 | pub struct BTreeMap<K, V> {
    | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1175:24
     |
1175 |         assert_eq!(map.first_entry().unwrap().key().id(), 1);
     |                        ^^^^^^^^^^^ method not found in `map::BTreeMap<Instance<'_>, ()>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1176:24
     |
1176 |         assert_eq!(map.last_entry().unwrap().key().id(), 2);
     |                        ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1207:24
     |
1207 |         assert_eq!(map.first_entry().unwrap().key().id(), 1);
     |                        ^^^^^^^^^^^ method not found in `map::BTreeMap<Instance<'_>, ()>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1208:24
     |
1208 |         assert_eq!(map.last_entry().unwrap().key().id(), 2);
     |                        ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1843:15
     |
1843 |     assert!(a.first_entry().is_none());
     |               ^^^^^^^^^^^ method not found in `map::BTreeMap<_, _>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1844:15
     |
1844 |     assert!(a.last_entry().is_none());
     |               ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1846:18
     |
1846 |     assert_eq!(a.first_entry().unwrap().key(), &1);
     |                  ^^^^^^^^^^^ method not found in `map::BTreeMap<{integer}, {integer}>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1847:18
     |
1847 |     assert_eq!(a.last_entry().unwrap().key(), &1);
     |                  ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1849:18
     |
1849 |     assert_eq!(a.first_entry().unwrap().key(), &1);
     |                  ^^^^^^^^^^^ method not found in `map::BTreeMap<{integer}, {integer}>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1850:18
     |
1850 |     assert_eq!(a.last_entry().unwrap().key(), &2);
     |                  ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1852:18
     |
1852 |     assert_eq!(a.first_entry().unwrap().key(), &0);
     |                  ^^^^^^^^^^^ method not found in `map::BTreeMap<{integer}, {integer}>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1853:18
     |
1853 |     assert_eq!(a.last_entry().unwrap().key(), &2);
     |                  ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1854:22
     |
1854 |     let (k1, v1) = a.first_entry().unwrap().remove_entry();
     |                      ^^^^^^^^^^^ method not found in `map::BTreeMap<{integer}, {integer}>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1857:22
     |
1857 |     let (k2, v2) = a.last_entry().unwrap().remove_entry();
     |                      ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1860:18
     |
1860 |     assert_eq!(a.first_entry().unwrap().key(), &1);
     |                  ^^^^^^^^^^^ method not found in `map::BTreeMap<{integer}, {integer}>`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_entry` not found for this

error[E0599]: no method named `last_entry` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:1861:18
     |
1861 |     assert_eq!(a.last_entry().unwrap().key(), &1);
     |                  ^^^^^^^^^^ help: there is an associated function with a similar name: `max_entry`
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_entry` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:2031:22
     |
2031 |     assert_eq!(*left.first_key_value().unwrap().0, 0);
     |
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `first_key_value` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:2032:23
     |
2032 |     assert_eq!(*right.first_key_value().unwrap().0, 1);
     |
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `first_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:2042:22
     |
2042 |     assert_eq!(*left.last_key_value().unwrap().0, last);
     |
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:2048:22
     |
2048 |     assert_eq!(*left.last_key_value().unwrap().0, last - 1);
     |
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `last_key_value` found for struct `map::BTreeMap` in the current scope
    --> library/alloc/src/collections/btree/map/tests.rs:2049:23
     |
2049 |     assert_eq!(*right.last_key_value().unwrap().0, last);
     |
    ::: library/alloc/src/collections/btree/map.rs:166:1
     |
166  | pub struct BTreeMap<K, V> {
166  | pub struct BTreeMap<K, V> {
     | ------------------------- method `last_key_value` not found for this

error[E0599]: no method named `first` found for struct `set::BTreeSet` in the current scope
   --> library/alloc/src/collections/btree/set/tests.rs:383:20
    |
383 |     assert_eq!(set.first().unwrap().id(), 1);
    |                    ^^^^^ method not found in `set::BTreeSet<Instance<'_>>`
   ::: library/alloc/src/collections/btree/set.rs:77:1
    |
77  | pub struct BTreeSet<T> {
77  | pub struct BTreeSet<T> {
    | ---------------------- method `first` not found for this

error[E0599]: the method `last` exists for struct `set::BTreeSet<Instance<'_>>`, but its trait bounds were not satisfied
    --> library/alloc/src/collections/btree/set/tests.rs:384:20
     |
384  |     assert_eq!(set.last().unwrap().id(), 2);
     |                    ^^^^ method cannot be called on `set::BTreeSet<Instance<'_>>` due to unsatisfied trait bounds
    ::: library/alloc/src/collections/btree/set.rs:77:1
     |
77   | pub struct BTreeSet<T> {
     | ----------------------
     | ----------------------
     | |
