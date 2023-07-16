
$ rg rustc_insignificant library -A1
library/alloc/src/collections/linked_list.rs
49:#[rustc_insignificant_dtor]
50-pub struct LinkedList<T> {

library/alloc/src/collections/vec_deque/mod.rs
97:#[rustc_insignificant_dtor]
98-pub struct VecDeque<

library/alloc/src/rc.rs
308:#[rustc_insignificant_dtor]
309-pub struct Rc<T: ?Sized> {

library/alloc/src/collections/btree/map.rs
171:#[rustc_insignificant_dtor]
172-pub struct BTreeMap<
--
381:#[rustc_insignificant_dtor]
382-pub struct IntoIter<

library/std/src/collections/hash/map.rs
212:#[rustc_insignificant_dtor]
213-pub struct HashMap<K, V, S = RandomState> {

library/alloc/src/vec/mod.rs
399:#[rustc_insignificant_dtor]
400-pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {

library/alloc/src/vec/into_iter.rs
29:#[rustc_insignificant_dtor]
30-pub struct IntoIter<

library/core/src/array/iter.rs
13:#[rustc_insignificant_dtor]
14-pub struct IntoIter<T, const N: usize> {
