rust
use std::cell::Cell;
use std::sync::MutexGuard;
use std::slice::{ChunksExactMut, ChunksMut};

fn foo()
where
    ChunksMut<'static, Cell<i32>>: Send,
    ChunksMut<'static, MutexGuard<'static, u32>>: Sync,
    ChunksExactMut<'static, Cell<i32>>: Send,
    ChunksExactMut<'static, MutexGuard<'static, u32>>: Sync,
{
}
