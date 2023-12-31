rust
struct VTable<DST: ?Sized> {
    _size: usize,
    _to_dst_ptr: fn(*mut ()) -> *mut DST,
}

trait HasVTableFor<DST: ?Sized + 'static> {
    const VTABLE: &'static VTable<DST>;
}

impl<T, DST: ?Sized + 'static> HasVTableFor<DST> for T {
    const VTABLE: &'static VTable<DST> = &VTable {
        _size: std::mem::size_of::<T>(),
        _to_dst_ptr: |_: *mut ()| unsafe { std::mem::zeroed() },
    };
}

pub fn push<DST: ?Sized + 'static, T>() {
    <T as HasVTableFor<DST>>::VTABLE;
}