
// write an iterator such that
struct Iter<E: Clone> { last_yielded: E, other: ... }

loop {
let i = Iter { ... };
let r = env::join_paths(i);
if r.is_ok() { break; }
// i.last_yielded is the invalid path that cannot be included into $PATH, remove and retry.
}
