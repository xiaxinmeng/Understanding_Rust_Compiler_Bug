 rust
pub struct NotSyncAtAll<T>(*mut T);

unsafe impl<T> Sync for NotSyncAtAll<T> {}

extern "C" {
    static FOO: NotSyncAtAll<u8>;
}
