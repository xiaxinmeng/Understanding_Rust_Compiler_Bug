
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections
src/libcollections/linked_list.rs:899:1: 906:2 error: type mismatch resolving `<linked_list::Iter<'_, &T> as core::iter::Iterator>::Item == &T`:
 expected &-ptr,
    found type parameter [E0271]
src/libcollections/linked_list.rs:899 impl<'a, T> IntoIterator for &'a LinkedList<T> {
src/libcollections/linked_list.rs:900     type Item = &'a T;
src/libcollections/linked_list.rs:901     type IntoIter = Iter<'a, Self::Item>;
src/libcollections/linked_list.rs:902 
src/libcollections/linked_list.rs:903     fn into_iter(self) -> Self::IntoIter {
src/libcollections/linked_list.rs:904         self.iter()
                                      ...
src/libcollections/linked_list.rs:899:1: 906:2 help: run `rustc --explain E0271` to see a detailed explanation
src/libcollections/linked_list.rs:899:1: 906:2 note: required by `core::iter::IntoIterator`
error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.collections] Error 101
