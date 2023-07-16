rust
#![feature(core_intrinsics)]
pub unsafe fn write<T>(dst: *mut T, src: T) {
    std::intrinsics::copy_nonoverlapping(&src as *const T, dst, 1);
    std::intrinsics::forget(src);
}
