rust
let m = MaybeUninit::<UnsafeCell<i32>>::uninit();
unsafe { UnsafeCell::raw_get(m.as_ptr()).write(5); }
let uc = unsafe { m.assume_init() };
