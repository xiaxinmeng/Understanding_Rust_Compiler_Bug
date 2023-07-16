plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error[E0425]: cannot find function `list_from` in this scope
   |
   |
25 |     let n = list_from(&[]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
26 |     let m = list_from(&[1, 2, 3]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
36 |     let n = list_from(&[nan]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
37 |     let m = list_from(&[nan]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
43 |     let n = list_from(&[nan]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
44 |     let one = list_from(&[1.0f64]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
50 |     let u = list_from(&[1.0f64, 2.0, nan]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
51 |     let v = list_from(&[1.0f64, 2.0, 3.0]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
57 |     let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
58 |     let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);


error[E0425]: cannot find function `list_from` in this scope
   |
   |
82 |     assert_eq!(a, list_from(&[1, 2, 3, 4]));


error[E0425]: cannot find function `list_from` in this scope
   |
   |
90 |     assert_eq!(a, list_from(&[1, 2, 3, 4, 5, 6]));

error[E0425]: cannot find function `catch_unwind` in this scope
   --> library/alloc/tests/linked_list.rs:325:5
    |
    |
325 |     catch_unwind(AssertUnwindSafe(|| drop(q.drain_filter(|_| true)))).ok();
    |
help: consider importing this function
    |
1   | use std::panic::catch_unwind;
1   | use std::panic::catch_unwind;
    |

error[E0425]: cannot find function, tuple struct or tuple variant `AssertUnwindSafe` in this scope
    |
    |
325 |     catch_unwind(AssertUnwindSafe(|| drop(q.drain_filter(|_| true)))).ok();
    |
help: consider importing one of these items
    |
1   | use core::panic::AssertUnwindSafe;
---
    |
1   | use std::panic::catch_unwind;
    |

error[E0425]: cannot find function, tuple struct or tuple variant `AssertUnwindSafe` in this scope
    |
357 |     catch_unwind(AssertUnwindSafe(|| {
    |                  ^^^^^^^^^^^^^^^^ not found in this scope
    |
---

error[E0425]: cannot find function `catch_unwind` in this scope
   --> library/alloc/tests/linked_list.rs:467:5
    |
467 |     catch_unwind(move || drop(q)).ok();
    |
help: consider importing this function
    |
1   | use std::panic::catch_unwind;
