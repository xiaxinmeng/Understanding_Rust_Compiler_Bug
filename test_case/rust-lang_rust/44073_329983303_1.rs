rust
union FatThin<T: ?Sized> {
    fat: *mut T,
    thin: *mut ()
}
let mut combined = FatThin { fat: ptr as *mut RcBox<T> };
combined.thin = &uninit.value as *const _ as *const ();
let base_ptr = &uninit.value as *const _ as usize;
let data_ptr = &(*combined.fat).data as *const _ as usize;
let offset = data_ptr - base_ptr;
