
error[E0080]: constant evaluation error
  --> C:\projects\rust\src\libstd\sys\windows\stack_overflow.rs:45:8
   |
45 |     if c::AddVectoredExceptionHandler(0, vectored_handler).is_null() {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no mir for `std::sys::windows::c::::AddVectoredExceptionHandler`
   |
note: inside call to `std::sys::windows::stack_overflow::init`
  --> C:\projects\rust\src\libstd\rt.rs:44:9
   |
44 |         sys::stack_overflow::init();
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside call to `std::rt::lang_start_internal`
  --> C:\projects\rust\src\libstd\rt.rs:74:5
   |
74 |     lang_start_internal(&move || main().report(), argc, argv)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
error: aborting due to previous error
