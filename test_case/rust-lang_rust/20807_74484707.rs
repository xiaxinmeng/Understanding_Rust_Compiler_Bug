 rust
struct Panicker<T>(T);
// Implement PartialOrd, Ord on Panicker so that it panics when compared with `<=`.

let mut heap = BinaryHeap::new();
let guard = Thread::scoped(|| {
    heap.push(Panicker(Vec::new()));
    heap.push(Panicker(Vec::new()));
});
guard.join().ok();

// Access heaps' first element here.
let zeroed_struct = heap.into_iter().next();

// BinaryHeap's sift_up will now panic on the first comparison, but not until after having replaced
// its first element with `zeroed()`.
// So we can create a zeroed-out version of any struct we want.
// This breaks many invariants:
// 1) A zeroed Vec means that Some(v) is the same as None, because the vec's ptr is marked NonZero.
// 2) We can zero a BTreeMap which will result in a btreemap with 0 for `b` and null for the
// root node, which will lead to an immediate violation since the insert/lookup
// methods assume root is not null.
