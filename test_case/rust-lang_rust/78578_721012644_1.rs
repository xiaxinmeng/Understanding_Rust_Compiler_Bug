rust
#![feature(const_mut_refs)]
#![feature(const_raw_ptr_deref)]
static mut FOO: i32 = 0;

struct SyncRawPtr<T>(*mut T);
unsafe impl<T> Sync for SyncRawPtr<T> {}

const fn make_evil() -> (&'static i32, SyncRawPtr<i32>) {
    let x: *mut i32 = unsafe { &mut FOO };
    (unsafe { &*x }, SyncRawPtr(x))
}

static EVIL: (&i32, SyncRawPtr<i32>) = {
    make_evil()
};
