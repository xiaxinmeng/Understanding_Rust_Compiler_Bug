rust
#![feature(core_intrinsics)]
pub unsafe fn write<T>(dst: *mut T, src: T) {
    std::intrinsics::move_val_init(&mut *dst, src)
}
