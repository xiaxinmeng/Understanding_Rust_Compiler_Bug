plain
travis_time:end:0fe3cff1:start=1549126511739528993,finish=1549126587260856363,duration=75521327370
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:20] 
[01:11:20] running 119 tests
[01:11:46] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:51] i......iii.i.....ii
[01:11:51] 
[01:11:51]  finished in 30.906
[01:11:51] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:13] 
[01:38:13] running 194 tests
[01:38:39] ......................i.F........................................................................... 100/194
[01:39:25] failures:
[01:39:25] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:39:25] 
[01:39:25] ---- [run-make] run-make-fulldeps/alloc-extern-crates stdout ----
[01:39:25] ---- [run-make] run-make-fulldeps/alloc-extern-crates stdout ----
[01:39:25] 
[01:39:25] error: make failed
[01:39:25] status: exit code: 2
[01:39:25] command: "make"
[01:39:25] stdout:
[01:39:25] ------------------------------------------
[01:39:25] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/alloc-extern-crates'
[01:39:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates  fakealloc.rs
[01:39:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates  --crate-type=rlib ../../../liballoc/lib.rs --cfg feature=\"external_crate\" --extern external=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-extern-crates/alloc-extern-crates/libfakealloc.rlib
[01:39:25] Makefile:4: recipe for target 'all' failed
[01:39:25] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/alloc-extern-crates'
[01:39:25] ------------------------------------------
[01:39:25] stderr:
[01:39:25] ------------------------------------------
[01:39:25] ------------------------------------------
[01:39:25] error[E0432]: unresolved import `if_arc`
[01:39:25]  --> ../../../liballoc/task.rs:6:9
[01:39:25]   |
[01:39:25] 6 | pub use if_arc::*;
[01:39:25]   |         ^^^^^^ did you mean `self::if_arc`?
[01:39:25] error[E0432]: unresolved import `SearchResult`
[01:39:25]  --> ../../../liballoc/collections/btree/search.rs:8:5
[01:39:25]   |
[01:39:25] 8 | use SearchResult::*;
[01:39:25] 8 | use SearchResult::*;
[01:39:25]   |     ^^^^^^^^^^^^ did you mean `self::SearchResult`?
[01:39:25] 
[01:39:25] error[E0432]: unresolved import `UnderflowResult`
[01:39:25]   --> ../../../liballoc/collections/btree/map.rs:20:5
[01:39:25]    |
[01:39:25] 20 | use UnderflowResult::*;
[01:39:25]    |     ^^^^^^^^^^^^^^^ did you mean `self::UnderflowResult`?
[01:39:25] error[E0432]: unresolved import `Entry`
[01:39:25]   --> ../../../liballoc/collections/btree/map.rs:21:5
[01:39:25]    |
[01:39:25] 21 | use Entry::*;
[01:39:25] 21 | use Entry::*;
[01:39:25]    |     ^^^^^ did you mean `self::Entry`?
[01:39:25] 
[01:39:25] error[E0432]: unresolved import `binary_heap`
[01:39:25]   --> ../../../liballoc/collections/mod.rs:26:9
[01:39:25] 26 | pub use binary_heap::BinaryHeap;
[01:39:25]    |         ^^^^^^^^^^^ did you mean `self::binary_heap`?
[01:39:25] 
[01:39:25] error[E0432]: unresolved import `btree_map`
[01:39:25] error[E0432]: unresolved import `btree_map`
[01:39:25]   --> ../../../liballoc/collections/mod.rs:30:9
[01:39:25]    |
[01:39:25] 30 | pub use btree_map::BTreeMap;
[01:39:25]    |         ^^^^^^^^^ did you mean `self::btree_map`?
[01:39:25] 
[01:39:25] error[E0432]: unresolved import `btree_set`
[01:39:25]   --> ../../../liballoc/collections/mod.rs:34:9
[01:39:25]    |
[01:39:25] 34 | pub use btree_set::BTreeSet;
[01:39:25]    |         ^^^^^^^^^ did you mean `self::btree_set`?
[01:39:25] error[E0432]: unresolved import `linked_list`
[01:39:25]   --> ../../../liballoc/collections/mod.rs:38:9
[01:39:25]    |
[01:39:25] 38 | pub use linked_list::LinkedList;
---
[01:39:25]    |
[01:39:25] 42 | pub use vec_deque::VecDeque;
[01:39:25]    |         ^^^^^^^^^ did you mean `self::vec_deque`?
[01:39:25] 
[01:39:25] error[E0432]: unresolved import `Fallibility`
[01:39:25]    --> ../../../liballoc/raw_vec.rs:627:5
[01:39:25] 627 | use Fallibility::*;
[01:39:25]     |     ^^^^^^^^^^^ did you mean `self::Fallibility`?
[01:39:25] 
[01:39:25] error[E0432]: unresolved import `ReserveStrategy`
---
[01:39:25]    |
[01:39:25] 16 | use Cow::*;
[01:39:25]    |     ^^^ did you mean `self::Cow`?
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Found` in this scope
[01:39:25]   --> ../../../liballoc/collections/btree/search.rs:23:13
[01:39:25]    |
[01:39:25] 23 |             Found(handle) => return Found(handle),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]    |
[01:39:25] 1  | use collections::btree::search::SearchResult::Found;
[01:39:25]    |
[01:39:25]    |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Found` in this scope
[01:39:25]   --> ../../../liballoc/collections/btree/search.rs:23:37
[01:39:25]    |
[01:39:25] 23 |             Found(handle) => return Found(handle),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]    |
[01:39:25] 1  | use collections::btree::search::SearchResult::Found;
[01:39:25]    |
[01:39:25]    |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `GoDown` in this scope
[01:39:25]   --> ../../../liballoc/collections/btree/search.rs:24:13
[01:39:25]    |
[01:39:25] 24 |             GoDown(handle) => match handle.force() {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]    |
[01:39:25]    |
[01:39:25] 1  | use collections::btree::search::SearchResult::GoDown;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `GoDown` in this scope
[01:39:25]   --> ../../../liballoc/collections/btree/search.rs:25:38
[01:39:25]    |
[01:39:25] 25 |                 Leaf(leaf) => return GoDown(leaf),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]    |
[01:39:25]    |
[01:39:25] 1  | use collections::btree::search::SearchResult::GoDown;
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Found` in this scope
[01:39:25]   --> ../../../liballoc/collections/btree/search.rs:42:24
[01:39:25]    |
[01:39:25]    |
[01:39:25] 42 |         (idx, true) => Found(
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]    |
[01:39:25] 1  | use collections::btree::search::SearchResult::Found;
[01:39:25]    |
[01:39:25]    |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]    --> ../../../liballoc/collections/btree/map.rs:448:13
[01:39:25]     |
[01:39:25] 448 |             Vacant(ref v) => f.debug_tuple("Entry")
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 1   | use collections::btree::map::Entry::Vacant;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]    --> ../../../liballoc/collections/btree/map.rs:451:13
[01:39:25]     |
[01:39:25] 451 |             Occupied(ref o) => f.debug_tuple("Entry")
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 1   | use collections::btree::map::Entry::Occupied;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]    --> ../../../liballoc/collections/btree/map.rs:693:13
[01:39:25]     |
[01:39:25] 693 |             Occupied(mut entry) => Some(entry.insert(value)),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 1   | use collections::btree::map::Entry::Occupied;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]    --> ../../../liballoc/collections/btree/map.rs:694:13
[01:39:25]     |
[01:39:25] 694 |             Vacant(entry) => {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 1   | use collections::btree::map::Entry::Vacant;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Occupied` in this scope
[01:39:25]    --> ../../../liballoc/collections/btree/map.rs:900:17
[01:39:25]     |
[01:39:25] 900 |                 Occupied(OccupiedEntry {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 1   | use collections::btree::map::Entry::Occupied;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Vacant` in this scope
[01:39:25]    --> ../../../liballoc/collections/btree/map.rs:907:17
[01:39:25]     |
[01:39:25] 907 |                 Vacant(VacantEntry {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 1   | use collections::btree::map::Entry::Vacant;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2112:13
[01:39:25]      |
[01:39:25] 2112 |             Occupied(entry) => entry.into_mut(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Occupied;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2113:13
[01:39:25]      |
[01:39:25] 2113 |             Vacant(entry) => entry.insert(default),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Vacant;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2135:13
[01:39:25]      |
[01:39:25] 2135 |             Occupied(entry) => entry.into_mut(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Occupied;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2136:13
[01:39:25]      |
[01:39:25] 2136 |             Vacant(entry) => entry.insert(default()),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Vacant;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2153:13
[01:39:25]      |
[01:39:25] 2153 |             Occupied(ref entry) => entry.key(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Occupied;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2154:13
[01:39:25]      |
[01:39:25] 2154 |             Vacant(ref entry) => entry.key(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Vacant;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2183:13
[01:39:25]      |
[01:39:25] 2183 |             Occupied(mut entry) => {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Occupied;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Occupied` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2185:17
[01:39:25] 2185 |                 Occupied(entry)
[01:39:25]      |                 ^^^^^^^^ not found in this scope
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Occupied;
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2187:13
[01:39:25]      |
[01:39:25] 2187 |             Vacant(entry) => Vacant(entry),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Vacant;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Vacant` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2187:30
[01:39:25]      |
[01:39:25] 2187 |             Vacant(entry) => Vacant(entry),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Vacant;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Occupied` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2211:13
[01:39:25]      |
[01:39:25] 2211 |             Occupied(entry) => entry.into_mut(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Occupied;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Vacant` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2212:13
[01:39:25]      |
[01:39:25] 2212 |             Vacant(entry) => entry.insert(Default::default()),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::Entry::Vacant;
[01:39:25]      |
[01:39:25]      |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `EmptyParent` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2505:17
[01:39:25]      |
[01:39:25] 2505 |                 EmptyParent(_) => unreachable!(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::EmptyParent;
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Merged` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2506:17
[01:39:25]      |
[01:39:25]      |
[01:39:25] 2506 |                 Merged(parent) => {
[01:39:25]      |                 ^^^^^^ not found in this scope
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::Merged;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Stole` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2515:17
[01:39:25]      |
[01:39:25] 2515 |                 Stole(_) => break,
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::Stole;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `AtRoot` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2535:16
[01:39:25] 2535 |         return AtRoot;
[01:39:25]      |                ^^^^^^ not found in this scope
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::AtRoot;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `EmptyParent` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2544:28
[01:39:25]      |
[01:39:25] 2544 |                     return EmptyParent(parent.into_node());
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::EmptyParent;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Merged` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2551:9
[01:39:25]      |
[01:39:25] 2551 |         Merged(handle.merge().into_node())
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::Merged;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Stole` in this scope
[01:39:25]     --> ../../../liballoc/collections/btree/map.rs:2558:9
[01:39:25]      |
[01:39:25] 2558 |         Stole(handle.into_node())
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]      |
[01:39:25]      |
[01:39:25] 1    | use collections::btree::map::UnderflowResult::Stole;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Fallible` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:391:59
[01:39:25]     |
[01:39:25] 391 |         self.reserve_internal(used_cap, needed_extra_cap, Fallible, Exact)
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 4   | use raw_vec::Fallibility::Fallible;
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Exact` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:391:69
[01:39:25]     |
[01:39:25]     |
[01:39:25] 391 |         self.reserve_internal(used_cap, needed_extra_cap, Fallible, Exact)
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 4   | use raw_vec::ReserveStrategy::Exact;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Infallible` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:415:65
[01:39:25]     |
[01:39:25] 415 |         match self.reserve_internal(used_cap, needed_extra_cap, Infallible, Exact) {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 4   | use raw_vec::Fallibility::Infallible;
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Exact` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:415:77
[01:39:25]     |
[01:39:25]     |
[01:39:25] 415 |         match self.reserve_internal(used_cap, needed_extra_cap, Infallible, Exact) {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 4   | use raw_vec::ReserveStrategy::Exact;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Fallible` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:439:59
[01:39:25]     |
[01:39:25] 439 |         self.reserve_internal(used_cap, needed_extra_cap, Fallible, Amortized)
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 4   | use raw_vec::Fallibility::Fallible;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Amortized` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:439:69
[01:39:25]     |
[01:39:25] 439 |         self.reserve_internal(used_cap, needed_extra_cap, Fallible, Amortized)
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 4   | use raw_vec::ReserveStrategy::Amortized;
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Infallible` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:495:65
[01:39:25]     |
[01:39:25]     |
[01:39:25] 495 |         match self.reserve_internal(used_cap, needed_extra_cap, Infallible, Amortized) {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 4   | use raw_vec::Fallibility::Infallible;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0425]: cannot find value `Amortized` in this scope
[01:39:25]    --> ../../../liballoc/raw_vec.rs:495:77
[01:39:25]     |
[01:39:25] 495 |         match self.reserve_internal(used_cap, needed_extra_cap, Infallible, Amortized) {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 4   | use raw_vec::ReserveStrategy::Amortized;
[01:39:25] 
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Borrowed` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:189:13
[01:39:25]     |
[01:39:25] 189 |             Borrowed(b) => Borrowed(b),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Borrowed;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Borrowed` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:189:28
[01:39:25]     |
[01:39:25] 189 |             Borrowed(b) => Borrowed(b),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Borrowed;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:190:13
[01:39:25]     |
[01:39:25] 190 |             Owned(ref o) => {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:192:17
[01:39:25]     |
[01:39:25] 192 |                 Owned(b.to_owned())
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:198:16
[01:39:25]     |
[01:39:25] 198 |         if let Owned(ref mut dest) = *self {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:199:20
[01:39:25]     |
[01:39:25] 199 |             if let Owned(ref o) = *source {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Borrowed` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:230:13
[01:39:25]     |
[01:39:25] 230 |             Borrowed(borrowed) => {
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Borrowed;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0425]: cannot find function `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:231:25
[01:39:25]     |
[01:39:25] 231 |                 *self = Owned(borrowed.to_owned());
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Borrowed` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:233:21
[01:39:25] 233 |                     Borrowed(..) => unreachable!(),
[01:39:25]     |                     ^^^^^^^^ not found in this scope
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Borrowed;
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:234:21
[01:39:25]     |
[01:39:25] 234 |                     Owned(ref mut owned) => owned,
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Owned` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:237:13
[01:39:25]     |
[01:39:25] 237 |             Owned(ref mut owned) => owned,
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
[01:39:25] 5   | use borrow::Cow::Owned;
[01:39:25]     |
[01:39:25]     |
[01:39:25] 
[01:39:25] error[E0531]: cannot find tuple struct/variant `Borrowed` in this scope
[01:39:25]    --> ../../../liballoc/borrow.rs:278:13
[01:39:25]     |
[01:39:25] 278 |             Borrowed(borrowed) => borrowed.to_owned(),
[01:39:25] help: possible candidate is found in another module, you can import it into scope
[01:39:25]     |
---
[01:39:25]   |     ^^^^^^^^^^^^^^^
[01:39:25]   |
[01:39:25]   = note: #[warn(unused_imports)] on by default
[01:39:25] 
[01:39:25] warning: unused import: `UnderflowResult::*`
[01:39:25]   --> ../../../liballoc/collections/btree/map.rs:20:5
[01:39:25]    |
[01:39:25] 20 | use UnderflowResult::*;
[01:39:25] 
[01:39:25] warning: unused import: `Entry::*`
[01:39:25]   --> ../../../liballoc/collections/btree/map.rs:21:5
[01:39:25]    |
[01:39:25]    |
[01:39:25] 21 | use Entry::*;
[01:39:25]    |     ^^^^^^^^
[01:39:25] 
[01:39:25] warning: unused import: `Fallibility::*`
[01:39:25]    --> ../../../liballoc/raw_vec.rs:627:5
[01:39:25] 627 | use Fallibility::*;
[01:39:25]     |     ^^^^^^^^^^^^^^
[01:39:25] 
[01:39:25] warning: unused import: `ReserveStrategy::*`
---
[01:39:25] error: aborting due to 70 previous errors
[01:39:25] 
[01:39:25] Some errors occurred: E0425, E0432, E0531.
[01:39:25] For more information about an error, try `rustc --explain E0425`.
[01:39:25] make[1]: *** [all] Error 1
[01:39:25] ------------------------------------------
[01:39:25] 
[01:39:25] thread '[run-make] run-make-fulldeps/alloc-extern-crates' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:39:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:39:25] test result: FAILED. 192 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:39:25] 
[01:39:25] 
[01:39:25] 
[01:39:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:39:25] 
[01:39:25] 
[01:39:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:25] Build completed unsuccessfully in 0:39:53
[01:39:25] Build completed unsuccessfully in 0:39:53
[01:39:25] make: *** [check] Error 1
[01:39:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04654594
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 18:36:02 UTC 2019
---
travis_time:end:28c34b14:start=1549132564175663706,finish=1549132564181238060,duration=5574354
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15fd2d6a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15297830
travis_time:start:15297830
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:033019b8
$ dmesg | grep -i kill
