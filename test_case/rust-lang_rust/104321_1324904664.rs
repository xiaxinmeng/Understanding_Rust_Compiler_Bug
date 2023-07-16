plain
  --> library/alloc/tests/autotraits.rs:38:29
   |
38 |       require_send_sync(async {
   |  _____________________________^
39 | |         let _v = None::<alloc::collections::btree_map::Iter<'_, &u32, &u32>>;
40 | |         async {}.await;
   | |_____^

error: requires `from_generator` lang_item
   --> library/alloc/tests/autotraits.rs:141:29
   --> library/alloc/tests/autotraits.rs:141:29
    |
141 |       require_send_sync(async {
    |  _____________________________^
142 | |         let _v = None::<alloc::collections::btree_set::BTreeSet<&u32>>;
143 | |         async {}.await;
    | |_____^

error: requires `from_generator` lang_item
   --> library/alloc/tests/autotraits.rs:189:29
   --> library/alloc/tests/autotraits.rs:189:29
    |
189 |       require_send_sync(async {
    |  _____________________________^
190 | |         let _v = None::<alloc::collections::binary_heap::BinaryHeap<&u32>>;
191 | |         async {}.await;
    | |_____^

error: requires `from_generator` lang_item
   --> library/alloc/tests/autotraits.rs:227:29
   --> library/alloc/tests/autotraits.rs:227:29
    |
227 |       require_send_sync(async {
    |  _____________________________^
228 | |         let _v = None::<alloc::collections::linked_list::Cursor<'_, &u32>>;
229 | |         async {}.await;
    | |_____^

error: requires `from_generator` lang_item
   --> library/alloc/tests/autotraits.rs:269:29
   --> library/alloc/tests/autotraits.rs:269:29
    |
269 |       require_send_sync(async {
    |  _____________________________^
270 | |         let _v = None::<alloc::collections::vec_deque::Drain<'_, &u32>>;
271 | |         async {}.await;
    | |_____^

error: could not compile `alloc` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: requires `from_generator` lang_item
  --> library/core/tests/future.rs:76:38
   |
76 |     async fn async_fn(_: impl Sized) {}

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:02:45
