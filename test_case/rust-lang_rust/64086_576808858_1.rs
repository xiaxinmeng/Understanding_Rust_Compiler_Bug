
   Compiling playground v0.0.1 (/playground)
error[E0277]: the type `std::cell::UnsafeCell<{integer}>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
  --> src/main.rs:12:5
   |
5  | fn assert_ref_unwind_safe<T: RefUnwindSafe>(t: T) {
   |    ----------------------    ------------- required by this bound in `assert_ref_unwind_safe`
...
12 |     assert_ref_unwind_safe(ParkingLotMutex::new(0));
   |     ^^^^^^^^^^^^^^^^^^^^^^ `std::cell::UnsafeCell<{integer}>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
   |
   = help: within `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, {integer}>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<{integer}>`
   = note: required because it appears within the type `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, {integer}>`
...

