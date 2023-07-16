 rust
struct RootedVecBox<T> {
 len: usize,
 cap: usize,
 rootsetptr: *const RootSetEntry,
 data: [T]
}
