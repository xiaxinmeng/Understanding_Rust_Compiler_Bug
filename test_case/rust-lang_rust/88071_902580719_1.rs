
error[E0277]: the trait bound `u32: Ord` is not satisfied
   --> src/lib.rs:10:19
    |
10  |             pool: BTreeSet::new(),
    |                   ^^^^^^^^^^^^^ the trait `Ord` is not implemented for `u32`
    |
note: required by `BTreeSet::<T>::new`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
    |
7   | impl PidPool where u32: Ord {
    |              ++++++++++++++
