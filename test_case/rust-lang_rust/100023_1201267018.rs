plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0412]: cannot find type `RChunksMut` in this scope
    --> library/core/tests/slice.rs:1207:9
     |
1207 |         RChunksMut<'static, Cell<i32>>: Send,
     |
    ::: /checkout/library/core/src/slice/iter.rs:1630:1
     |
     |
1630 | pub struct ChunksMut<'a, T: 'a> {
     | ------------------------------- similarly named struct `ChunksMut` defined here
help: a struct with a similar name exists
     |
     |
1207 |         ChunksMut<'static, Cell<i32>>: Send,
help: consider importing one of these items
     |
1    | use core::slice::RChunksMut;
     |
---

error[E0412]: cannot find type `RChunksMut` in this scope
    --> library/core/tests/slice.rs:1208:9
     |
1208 |         RChunksMut<'static, MutexGuard<'static, u32>>: Sync,
     |
    ::: /checkout/library/core/src/slice/iter.rs:1630:1
     |
     |
1630 | pub struct ChunksMut<'a, T: 'a> {
     | ------------------------------- similarly named struct `ChunksMut` defined here
help: a struct with a similar name exists
     |
     |
1208 |         ChunksMut<'static, MutexGuard<'static, u32>>: Sync,
help: consider importing one of these items
     |
1    | use core::slice::RChunksMut;
     |
---

error[E0412]: cannot find type `RChunksExactMut` in this scope
    --> library/core/tests/slice.rs:1209:9
     |
1209 |         RChunksExactMut<'static, Cell<i32>>: Send,
     |
    ::: /checkout/library/core/src/slice/iter.rs:1978:1
     |
     |
1978 | pub struct ChunksExactMut<'a, T: 'a> {
     | ------------------------------------ similarly named struct `ChunksExactMut` defined here
help: a struct with a similar name exists
     |
     |
1209 |         ChunksExactMut<'static, Cell<i32>>: Send,
help: consider importing one of these items
     |
1    | use core::slice::RChunksExactMut;
     |
---

error[E0412]: cannot find type `RChunksExactMut` in this scope
    --> library/core/tests/slice.rs:1210:9
     |
1210 |         RChunksExactMut<'static, MutexGuard<'static, u32>>: Sync,
     |
    ::: /checkout/library/core/src/slice/iter.rs:1978:1
     |
     |
1978 | pub struct ChunksExactMut<'a, T: 'a> {
     | ------------------------------------ similarly named struct `ChunksExactMut` defined here
help: a struct with a similar name exists
     |
     |
1210 |         ChunksExactMut<'static, MutexGuard<'static, u32>>: Sync,
help: consider importing one of these items
     |
1    | use core::slice::RChunksExactMut;
     |
