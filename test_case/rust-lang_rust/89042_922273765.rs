
error[E0423]: expected value, found enum `std::ffi::c_void`
  --> src/lib.rs:46:11
   |
46 |     match std::ffi::c_void {};
   |           ^^^^^^^^^^^^^^^^
   |
help: you might have meant to use one of the following enum variants
   |
46 |     match std::os::raw::c_void::__variant1 {};
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
46 |     match std::os::raw::c_void::__variant2 {};
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
