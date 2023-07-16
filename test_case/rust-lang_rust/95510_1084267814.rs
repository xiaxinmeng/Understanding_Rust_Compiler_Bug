plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: `std::array::<impl Deref for [T; N]>::deref` is not yet stable as a const fn
  |
  |
6 |     unsafe { (DATA.as_ptr() as *const u8).add(1) as *const u16 }
  |
  = help: add `#![feature(const_array_deref)]` to the crate attributes to enable


error: `std::array::<impl DerefMut for [T; N]>::deref_mut` is not yet stable as a const fn
   |
   |
70 |             let unaligned_ptr = (two_aligned.as_mut_ptr() as *mut u8).add(1) as *mut u16;
   |
   = help: add `#![feature(const_array_deref)]` to the crate attributes to enable


error: `std::array::<impl DerefMut for [T; N]>::deref_mut` is not yet stable as a const fn
   |
   |
94 |             let unaligned_ptr = (two_aligned.as_mut_ptr() as *mut u8).add(1) as *mut u16;
   |
   = help: add `#![feature(const_array_deref)]` to the crate attributes to enable


error: `std::array::<impl DerefMut for [T; N]>::deref_mut` is not yet stable as a const fn
   |
46 |             write_bytes(arr.as_mut_ptr(), 0, 2);
   |                         ^^^^^^^^^^^^^^^^
   |
   |
   = help: add `#![feature(const_array_deref)]` to the crate attributes to enable

error: `std::array::<impl DerefMut for [T; N]>::deref_mut` is not yet stable as a const fn
   |
58 |             write_bytes(arr.as_mut_ptr(), 1, 2);
   |                         ^^^^^^^^^^^^^^^^
   |
   |
   = help: add `#![feature(const_array_deref)]` to the crate attributes to enable

error: `std::array::<impl DerefMut for [T; N]>::deref_mut` is not yet stable as a const fn
    |
    |
257 |         let ptr = xs.as_mut_ptr();
    |
    = help: add `#![feature(const_array_deref)]` to the crate attributes to enable


error: `std::array::<impl Deref for [T; N]>::deref` is not yet stable as a const fn
    |
    |
259 |             ptr.write_bytes(5u8, xs.len());
    |
    = help: add `#![feature(const_array_deref)]` to the crate attributes to enable

error: could not compile `core` due to 7 previous errors
