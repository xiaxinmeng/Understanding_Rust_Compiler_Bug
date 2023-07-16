plain
    Checking std v0.0.0 (/checkout/library/std)
error[E0308]: mismatched types
   --> library/alloc/benches/btree/map.rs:293:45
    |
283 | fn bench_range<F, R>(b: &mut Bencher, f: F)
    |                   - this type parameter
...
293 |                 let _ = black_box(map.range(f(i, j)));
    |                                             |
    |                                             expected reference, found type parameter `R`
    |                                             expected reference, found type parameter `R`
    |                                             help: consider borrowing here: `&f(i, j)`
    = note:   expected reference `&_`
            found type parameter `R`

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error[E0308]: mismatched types
    --> library/alloc/src/collections/btree/map/tests.rs:1660:17
     |
1660 |         v.range(..)
     |                 |
     |                 expected reference, found struct `RangeFull`
     |                 help: consider borrowing here: `&(..)`
     |
     |
     = note: expected reference `&_`
                   found struct `RangeFull`

error[E0308]: mismatched types
    --> library/alloc/src/collections/btree/map/tests.rs:1664:21
     |
1664 |         v.range_mut(..)
     |                     |
     |                     expected reference, found struct `RangeFull`
     |                     help: consider borrowing here: `&(..)`
     |
     |
     = note: expected reference `&_`
                   found struct `RangeFull`

error[E0308]: mismatched types
    --> library/alloc/src/collections/btree/map/tests.rs:1729:17
     |
1729 |         v.range(..)
     |                 |
     |                 expected reference, found struct `RangeFull`
     |                 help: consider borrowing here: `&(..)`
     |
     |
     = note: expected reference `&_`
                   found struct `RangeFull`

error[E0308]: mismatched types
    --> library/alloc/src/collections/btree/map/tests.rs:1733:21
     |
1733 |         v.range_mut(..)
     |                     |
     |                     expected reference, found struct `RangeFull`
     |                     help: consider borrowing here: `&(..)`
     |
     |
     = note: expected reference `&_`
                   found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/set/tests.rs:548:17
    |
548 |         v.range(..)
    |                 |
    |                 expected reference, found struct `RangeFull`
    |                 help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/set/tests.rs:587:17
    |
587 |         v.range(..)
    |                 |
    |                 expected reference, found struct `RangeFull`
    |                 help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:572:15
    |
571 | fn range_keys(map: &BTreeMap<i32, i32>, range: impl RangeBounds<i32>) -> Vec<i32> {
    |                                                --------------------- this type parameter
572 |     map.range(range)
    |               |
    |               |
    |               expected reference, found type parameter `impl RangeBounds<i32>`
    |               help: consider borrowing here: `&range`
    = note:   expected reference `&_`
    = note:   expected reference `&_`
            found type parameter `impl RangeBounds<i32>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:262:26
    |
    |
262 |     assert_eq!(map.range(..).next(), None);
    |                          |
    |                          expected reference, found struct `RangeFull`
    |                          help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:263:26
    |
263 |     assert_eq!(map.range(..1).next(), None);
    |                          |
    |                          expected reference, found struct `RangeTo`
    |                          help: consider borrowing here: `&(..1)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeTo<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:264:26
    |
    |
264 |     assert_eq!(map.range(1..).next(), None);
    |                          |
    |                          expected reference, found struct `RangeFrom`
    |                          help: consider borrowing here: `&(1..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFrom<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:265:26
    |
    |
265 |     assert_eq!(map.range(1..=1).next(), None);
    |                          |
    |                          expected reference, found struct `RangeInclusive`
    |                          expected reference, found struct `RangeInclusive`
    |                          help: consider borrowing here: `&(1..=1)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeInclusive<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:266:26
    |
    |
266 |     assert_eq!(map.range(1..2).next(), None);
    |                          |
    |                          expected reference, found struct `core::ops::Range`
    |                          help: consider borrowing here: `&(1..2)`
    |
    |
    = note: expected reference `&_`
                  found struct `core::ops::Range<{integer}>`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:326:26
    |
326 |     assert_eq!(map.range(..).next(), None);
    |                          |
    |                          expected reference, found struct `RangeFull`
    |                          help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:327:26
    |
327 |     assert_eq!(map.range(..1).next(), None);
    |                          |
    |                          expected reference, found struct `RangeTo`
    |                          help: consider borrowing here: `&(..1)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeTo<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:328:26
    |
    |
328 |     assert_eq!(map.range(1..).next(), None);
    |                          |
    |                          expected reference, found struct `RangeFrom`
    |                          help: consider borrowing here: `&(1..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFrom<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:329:26
    |
    |
329 |     assert_eq!(map.range(1..=1).next(), None);
    |                          |
    |                          expected reference, found struct `RangeInclusive`
    |                          expected reference, found struct `RangeInclusive`
    |                          help: consider borrowing here: `&(1..=1)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeInclusive<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:330:26
    |
    |
330 |     assert_eq!(map.range(1..2).next(), None);
    |                          |
    |                          expected reference, found struct `core::ops::Range`
    |                          help: consider borrowing here: `&(1..2)`
    |
    |
    = note: expected reference `&_`
                  found struct `core::ops::Range<{integer}>`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:542:24
    |
542 |     assert_eq!(a.range(..).min(), None);
    |                        |
    |                        expected reference, found struct `RangeFull`
    |                        help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:543:24
    |
543 |     assert_eq!(a.range(..).max(), None);
    |                        |
    |                        expected reference, found struct `RangeFull`
    |                        help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:544:28
    |
544 |     assert_eq!(a.range_mut(..).min(), None);
    |                            |
    |                            expected reference, found struct `RangeFull`
    |                            help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:545:28
    |
545 |     assert_eq!(a.range_mut(..).max(), None);
    |                            |
    |                            expected reference, found struct `RangeFull`
    |                            help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:558:24
    |
558 |     assert_eq!(a.range(..).min(), Some((&1, &42)));
    |                        |
    |                        expected reference, found struct `RangeFull`
    |                        help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:559:24
    |
559 |     assert_eq!(a.range(..).max(), Some((&2, &24)));
    |                        |
    |                        expected reference, found struct `RangeFull`
    |                        help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:560:28
    |
560 |     assert_eq!(a.range_mut(..).min(), Some((&1, &mut 42)));
    |                            |
    |                            expected reference, found struct `RangeFull`
    |                            help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:561:28
    |
561 |     assert_eq!(a.range_mut(..).max(), Some((&2, &mut 24)));
    |                            |
    |                            expected reference, found struct `RangeFull`
    |                            help: consider borrowing here: `&(..)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeFull`

error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:723:21
    |
723 |     check(map.range(..=100), map.range(..101));
    |                     |
    |                     expected reference, found struct `RangeToInclusive`
    |                     expected reference, found struct `RangeToInclusive`
    |                     help: consider borrowing here: `&(..=100)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeToInclusive<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:723:40
    |
    |
723 |     check(map.range(..=100), map.range(..101));
    |                                        |
    |                                        expected reference, found struct `RangeTo`
    |                                        help: consider borrowing here: `&(..101)`
    |
    |
    = note: expected reference `&_`
                  found struct `RangeTo<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:724:21
    |
    |
724 |     check(map.range(5..=8), vec![(&5, &5), (&6, &6), (&7, &7), (&8, &8)]);
    |                     |
    |                     expected reference, found struct `RangeInclusive`
    |                     expected reference, found struct `RangeInclusive`
    |                     help: consider borrowing here: `&(5..=8)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeInclusive<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:725:21
    |
    |
725 |     check(map.range(-1..=2), vec![(&1, &1), (&2, &2)]);
    |                     |
    |                     expected reference, found struct `RangeInclusive`
    |                     expected reference, found struct `RangeInclusive`
    |                     help: consider borrowing here: `&(-1..=2)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeInclusive<{integer}>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:733:26
    |
    |
733 |     assert_eq!(map.range(max..=max).collect::<Vec<_>>(), &[(&max, &0)]);
    |                          |
    |                          expected reference, found struct `RangeInclusive`
    |                          expected reference, found struct `RangeInclusive`
    |                          help: consider borrowing here: `&(max..=max)`
    = note: expected reference `&_`
                  found struct `RangeInclusive<usize>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:739:26
    |
739 |     assert_eq!(map.range((Included(2), Excluded(2))).next(), None);
    |                          |
    |                          expected reference, found tuple
    |                          expected reference, found tuple
    |                          help: consider borrowing here: `&(Included(2), Excluded(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:740:26
    |
    |
740 |     assert_eq!(map.range((Excluded(2), Included(2))).next(), None);
    |                          |
    |                          expected reference, found tuple
    |                          expected reference, found tuple
    |                          help: consider borrowing here: `&(Excluded(2), Included(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:747:23
    |
    |
747 |     let _ = map.range((Excluded(2), Excluded(2)));
    |                       |
    |                       expected reference, found tuple
    |                       expected reference, found tuple
    |                       help: consider borrowing here: `&(Excluded(2), Excluded(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:754:23
    |
    |
754 |     let _ = map.range((Included(3), Included(2)));
    |                       |
    |                       expected reference, found tuple
    |                       expected reference, found tuple
    |                       help: consider borrowing here: `&(Included(3), Included(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:761:23
    |
    |
761 |     let _ = map.range((Included(3), Excluded(2)));
    |                       |
    |                       expected reference, found tuple
    |                       expected reference, found tuple
    |                       help: consider borrowing here: `&(Included(3), Excluded(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:768:23
    |
    |
768 |     let _ = map.range((Excluded(3), Included(2)));
    |                       |
    |                       expected reference, found tuple
    |                       expected reference, found tuple
    |                       help: consider borrowing here: `&(Excluded(3), Included(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:775:23
    |
    |
775 |     let _ = map.range((Excluded(3), Excluded(2)));
    |                       |
    |                       expected reference, found tuple
    |                       expected reference, found tuple
    |                       help: consider borrowing here: `&(Excluded(3), Excluded(2))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<{integer}>, Bound<{integer}>)`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:786:27
    |
    |
786 |         let _ = map.range(Cyclic3::C..=Cyclic3::A);
    |                           |
    |                           expected reference, found struct `RangeInclusive`
    |                           expected reference, found struct `RangeInclusive`
    |                           help: consider borrowing here: `&(Cyclic3::C..=Cyclic3::A)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeInclusive<Cyclic3>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:827:23
    |
    |
827 |     let _ = map.range(EvilTwin(5)..=EvilTwin(7));
    |                       |
    |                       expected reference, found struct `RangeInclusive`
    |                       expected reference, found struct `RangeInclusive`
    |                       help: consider borrowing here: `&(EvilTwin(5)..=EvilTwin(7))`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `RangeInclusive<EvilTwin>`
error[E0308]: mismatched types
   --> library/alloc/src/collections/btree/map/tests.rs:837:33
    |
    |
837 |         let mut kvs = map.range((min, max)).map(|(&k, &v)| (k, v));
    |                                 |
    |                                 expected reference, found tuple
    |                                 expected reference, found tuple
    |                                 help: consider borrowing here: `&(min, max)`
    = note: expected reference `&_`
    = note: expected reference `&_`
                   found tuple `(Bound<&u32>, Bound<&u32>)`
