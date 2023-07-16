
error[E0720]: opaque type expands to a recursive type
  --> src/main.rs:26:46
   |
26 |     pub async fn build_dependencies(self) -> DependenciesBuilt {
   |                                              ^^^^^^^^^^^^^^^^^ expands to self-referential type
   |
   = note: expanded type is `std::future::GenFuture<[static generator@src/main.rs:26:64: 37:6 {fn(std::ops::Range<i32>) -> <std::ops::Range<i32> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<i32> as std::iter::IntoIterator>::into_iter}, i32, std::ops::Range<i32>, Builder, impl std::future::Future, (), DataLoaded, impl std::future::Future, DependenciesFetched, impl std::future::Future}]>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0720`.
