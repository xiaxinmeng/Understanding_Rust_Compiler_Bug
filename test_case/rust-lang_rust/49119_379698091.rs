
[00:03:32] error[E0432]: unresolved import `std::num::NonZeroU32`
[00:03:32]   --> librustc_data_structures/obligation_forest/node_index.rs:11:5
[00:03:32]    |
[00:03:32] 11 | use std::num::NonZeroU32;
[00:03:32]    |     ^^^^^^^^^^^^^^^^^^^^ no `NonZeroU32` in `num`
[00:03:32] 
[00:03:32] error[E0204]: the trait `Copy` may not be implemented for this type
[00:03:32]   --> librustc_data_structures/obligation_forest/node_index.rs:14:10
[00:03:32]    |
[00:03:32] 14 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
[00:03:32]    |          ^^^^
[00:03:32] 15 | pub struct NodeIndex {
[00:03:32] 16 |     index: NonZeroU32,
[00:03:32]    |     ----------------- this field does not implement `Copy`
