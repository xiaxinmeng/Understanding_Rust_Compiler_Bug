
doc tests for: C:\projects\rust\src/doc/nomicon\src\vec-final.md
running 1 test
test C:\projects\rust\src/doc/nomicon\src\vec-final.md - The_Final_Code (line 3) ... FAILED
failures:
---- C:\projects\rust\src/doc/nomicon\src\vec-final.md - The_Final_Code (line 3) stdout ----
error[E0599]: no method named `alloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\vec-final.md:37:34
   |
35 |                 let ptr = Global.alloc(Layout::array::<T>(1).unwrap());
   |                                  ^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
6  | use std::alloc::Alloc;
   |
error[E0599]: no method named `realloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\vec-final.md:41:34
   |
39 |                 let ptr = Global.realloc(self.ptr.as_ptr() as *mut _,
   |                                  ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
6  | use std::alloc::Alloc;
   |
error[E0061]: this function takes 1 parameter but 0 parameters were supplied
  --> C:\projects\rust\src/doc/nomicon\src\vec-final.md:49:17
   |
47 |                 oom()
   |                 ^^^^^ expected 1 parameter
error[E0599]: no method named `dealloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\vec-final.md:63:24
   |
61 |                 Global.dealloc(self.ptr.as_ptr() as *mut _,
   |                        ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
6  | use std::alloc::Alloc;
   |
thread 'C:\projects\rust\src/doc/nomicon\src\vec-final.md - The_Final_Code (line 3)' panicked at 'couldn't compile the test', librustdoc\test.rs:325:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    C:\projects\rust\src/doc/nomicon\src\vec-final.md - The_Final_Code (line 3)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
