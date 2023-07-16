plain
[00:49:30] .................................................................................................... 2200/4605
[00:49:34] ...................i................................................................................ 2300/4605
[00:49:38] .................................................................................................... 2400/4605
[00:49:41] .................................................................................................... 2500/4605
[00:49:45] ................................iiiiiiiii........................................................... 2600/4605
[00:49:51] .................................................................................................... 2800/4605
[00:49:55] .................................................................................................... 2900/4605
[00:49:58] ......................................................i............................................. 3000/4605
[00:50:01] .................................................................................................... 3100/4605
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:39] 
[01:02:39] running 111 tests
[01:02:43] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:02:43] ..iiii.....
[01:02:43] 
[01:02:43]  finished in 3.536
[01:02:43] travis_fold:end:test_codegen

---
[01:22:20] travis_fold:start:test_stage1-rustc_data_structures
travis_time:start:test_stage1-rustc_data_structures
Testing rustc_data_structures stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:20]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[01:22:23] error[E0599]: no function or associated item named `new` found for type `snapshot_map::SnapshotMap<_, _>` in the current scope
[01:22:23]   --> librustc_data_structures/snapshot_map/test.rs:15:19
[01:22:23]    |
[01:22:23] 15 |       let mut map = SnapshotMap::new();
[01:22:23]    |                     ^^^^^^^^^^^^^^^^ function or associated item not found in `snapshot_map::SnapshotMap<_, _>`
[01:22:23]    | 
[01:22:23]   ::: librustc_data_structures/snapshot_map/mod.rs:19:1
[01:22:23]    |
[01:22:23] 19 | / pub struct SnapshotMap<K, V>
[01:22:23] 20 | |     where K: Hash + Clone + Eq
[01:22:23] 21 | | {
[01:22:23] 22 | |     map: FxHashMap<K, V>,
[01:22:23] 23 | |     undo_log: Vec<UndoLog<K, V>>,
[01:22:23] 24 | | }
[01:22:23]    | |_- function or associated item `new` not found for this
[01:22:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]    = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]            candidate #1: `indexed_vec::Idx`
[01:22:23]            candidate #2: `ena::unify::UnificationStore`
[01:22:23]            candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `snapshot_map::SnapshotMap<_, _>` in the current scope
[01:22:23]   --> librustc_data_structures/snapshot_map/test.rs:32:19
[01:22:23]    |
[01:22:23] 32 |       let mut map = SnapshotMap::new();
[01:22:23]    |                     ^^^^^^^^^^^^^^^^ function or associated item not found in `snapshot_map::SnapshotMap<_, _>`
[01:22:23]    | 
[01:22:23]   ::: librustc_data_structures/snapshot_map/mod.rs:19:1
[01:22:23]    |
[01:22:23] 19 | / pub struct SnapshotMap<K, V>
[01:22:23] 20 | |     where K: Hash + Clone + Eq
[01:22:23] 21 | | {
[01:22:23] 22 | |     map: FxHashMap<K, V>,
[01:22:23] 23 | |     undo_log: Vec<UndoLog<K, V>>,
[01:22:23] 24 | | }
[01:22:23]    | |_- function or associated item `new` not found for this
[01:22:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]    = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]            candidate #1: `indexed_vec::Idx`
[01:22:23]            candidate #2: `ena::unify::UnificationStore`
[01:22:23]            candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `snapshot_map::SnapshotMap<_, _>` in the current scope
[01:22:23]   --> librustc_data_structures/snapshot_map/test.rs:41:19
[01:22:23]    |
[01:22:23] 41 |       let mut map = SnapshotMap::new();
[01:22:23]    |                     ^^^^^^^^^^^^^^^^ function or associated item not found in `snapshot_map::SnapshotMap<_, _>`
[01:22:23]    | 
[01:22:23]   ::: librustc_data_structures/snapshot_map/mod.rs:19:1
[01:22:23]    |
[01:22:23] 19 | / pub struct SnapshotMap<K, V>
[01:22:23] 20 | |     where K: Hash + Clone + Eq
[01:22:23] 21 | | {
[01:22:23] 22 | |     map: FxHashMap<K, V>,
[01:22:23] 23 | |     undo_log: Vec<UndoLog<K, V>>,
[01:22:23] 24 | | }
[01:22:23]    | |_- function or associated item `new` not found for this
[01:22:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]    = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]    = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]            candidate #1: `indexed_vec::Idx`
[01:22:23]            candidate #2: `ena::unify::UnificationStore`
[01:22:23]            candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 492 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 503 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 533 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 554 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 581 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 600 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 617 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 639 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 662 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 695 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 718 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 737 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 757 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 777 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 799 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] 
[01:22:23] error[E0599]: no function or associated item named `new` found for type `transitive_relation::TransitiveRelation<_>` in the current scope
[01:22:23]     |
[01:22:23]     |
[01:22:23] 22  | pub struct TransitiveRelation<T: Clone + Debug + Eq + Hash> {
[01:22:23]     | ----------------------------------------------------------- function or associated item `new` not found for this
[01:22:23] ...
[01:22:23] 837 |     let mut relation = TransitiveRelation::new();
[01:22:23]     |                        ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `transitive_relation::TransitiveRelation<_>`
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = help: items from traits can only be used if the trait is implemented and in scope
[01:22:23]     = note: the following traits define an item `new`, perhaps you need to implement one of them:
[01:22:23]             candidate #1: `indexed_vec::Idx`
[01:22:23]             candidate #2: `ena::unify::UnificationStore`
[01:22:23]             candidate #3: `rand::distributions::uniform::UniformSampler`
[01:22:23] error: aborting due to 19 previous errors
[01:22:23] 
[01:22:23] For more information about this error, try `rustc --explain E0599`.
[01:22:23] error: Could not compile `rustc_data_structures`.
[01:22:23] error: Could not compile `rustc_data_structures`.
[01:22:23] 
[01:22:23] To learn more, run the command again with --verbose.
[01:22:23] 
[01:22:23] 
[01:22:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:22:23] 
[01:22:23] 
[01:22:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:24] Build completed unsuccessfully in 0:37:34
[01:22:24] Build completed unsuccessfully in 0:37:34
[01:22:24] make: *** [check] Error 1
[01:22:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f78d790
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1576b03c:start=1539765388121149076,finish=1539765388127778924,duration=6629848
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15b68110
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f61a3ee
travis_time:start:0f61a3ee
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00412986
$ dmesg | grep -i kill
