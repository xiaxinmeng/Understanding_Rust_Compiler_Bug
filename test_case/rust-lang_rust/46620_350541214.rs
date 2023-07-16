
[00:04:17] error: unused import: `std::collections::BTreeMap`
[00:04:17]   --> /checkout/src/libtest/lib.rs:60:5
[00:04:17]    |
[00:04:17] 60 | use std::collections::BTreeMap;
[00:04:17]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:17]    |
[00:04:17] note: lint level defined here
[00:04:17]   --> /checkout/src/libtest/lib.rs:35:9
[00:04:17]    |
[00:04:17] 35 | #![deny(warnings)]
[00:04:17]    |         ^^^^^^^^
[00:04:17]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:04:17] 
[00:04:17] error[E0609]: no field `metrics` on type `&mut ConsoleTestState<T>`
[00:04:17]    --> /checkout/src/libtest/lib.rs:892:28
[00:04:17]     |
[00:04:17] 892 |                         st.metrics.insert_metric(test.name.as_slice(),
[00:04:17]     |                            ^^^^^^^
[00:04:17] 
[00:04:17] error: aborting due to 2 previous errors
[00:04:17] 
[00:04:17] error: Could not compile `test`.
