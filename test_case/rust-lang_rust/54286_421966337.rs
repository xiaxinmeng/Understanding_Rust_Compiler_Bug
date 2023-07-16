plain
[00:52:49] ....................................................................................................
[00:52:51] .....................................................i..............................................
[00:52:54] ....................................................................................................
[00:52:57] ....................................................................................................
[00:53:00] .iiiiiiiii..........................................................................................
[00:53:06] ....................................................................................................
[00:53:09] ..................................................................................i.................
[00:53:12] ....................................................................................................
[00:53:15] ....................................i.i..ii.........................................................
---
[01:17:03] travis_fold:start:test_stage1-rustc_data_structures
travis_time:start:test_stage1-rustc_data_structures
Testing rustc_data_structures stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:04]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[01:17:05] error[E0599]: no method named `merge` found for type `bit_set::BitSet<usize>` in the current scope
[01:17:05]    --> librustc_data_structures/bit_set.rs:859:18
[01:17:05]     |
[01:17:05] 28  | pub struct BitSet<T: Idx> {
[01:17:05]     | ------------------------- method `merge` not found for this
[01:17:05] ...
[01:17:05] 859 |     assert!(set1.merge(&set2));
[01:17:05] 
[01:17:05] 
[01:17:05] error[E0599]: no method named `merge` found for type `bit_set::BitSet<usize>` in the current scope
[01:17:05]    --> librustc_data_structures/bit_set.rs:860:19
[01:17:05]     |
[01:17:05] 28  | pub struct BitSet<T: Idx> {
[01:17:05]     | ------------------------- method `merge` not found for this
[01:17:05] ...
[01:17:05] 860 |     assert!(!set1.merge(&set2));
[01:17:05] 
[01:17:05] ...
[01:17:05] ...
[01:17:05] 921 |     let intersection = matrix.intersection(2, 65);
[01:17:05]     |
[01:17:05]     = help: did you mean `intersect_rows`?
[01:17:05] 
[01:17:05] 
[01:17:05] error[E0599]: no method named `merge` found for type `bit_set::BitMatrix<usize, usize>` in the current scope
[01:17:05]    --> librustc_data_structures/bit_set.rs:932:12
[01:17:05]     |
[01:17:05] 560 | pub struct BitMatrix<R: Idx, C: Idx> {
[01:17:05]     | ------------------------------------ method `merge` not found for this
[01:17:05] ...
[01:17:05] 932 |     matrix.merge(3, 5);
[01:17:05] 
[01:17:05] 
[01:17:05] error[E0599]: no method named `merge` found for type `bit_set::SparseBitMatrix<usize, usize>` in the current scope
[01:17:05]    --> librustc_data_structures/bit_set.rs:974:12
[01:17:05]     |
[01:17:05] 677 | / pub struct SparseBitMatrix<R, C>
[01:17:05] 678 | | where
[01:17:05] 679 | |     R: Idx,
[01:17:05] 680 | |     C: Idx,
[01:17:05] ...   |
[01:17:05] 683 | |     rows: IndexVec<R, Option<BitSet<C>>>,
[01:17:05] 684 | | }
[01:17:05]     | |_- method `merge` not found for this
[01:17:05] ...
[01:17:05] 974 |       matrix.merge(3, 5);
[01:17:05] 
travis_time:end:20c3dbc8:start=1537176877052734143,finish=1537181504494778294,duration=4627442044151

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
