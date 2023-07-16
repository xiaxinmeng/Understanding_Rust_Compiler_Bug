
doc tests for: C:\projects\rust\src/doc/nomicon\src\destructors.md
running 6 tests
test C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 28) ... FAILED
test C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 54) ... FAILED
test C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 6) ... ignored
test C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 125) ... FAILED
test C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 107) ... ok
test C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 93) ... ok
failures:
---- C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 28) stdout ----
error[E0599]: no method named `dealloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\destructors.md:41:20
   |
14 |             Global.dealloc(self.ptr.as_ptr() as *mut _, Layout::new::<T>())
   |                    ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
4  | use std::alloc::Alloc;
   |
thread 'C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 28)' panicked at 'couldn't compile the test', librustdoc\test.rs:325:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.
---- C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 54) stdout ----
error[E0599]: no method named `dealloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\destructors.md:67:20
   |
14 |             Global.dealloc(self.ptr.as_ptr() as *mut _, Layout::new::<T>());
   |                    ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
4  | use std::alloc::Alloc;
   |
error[E0599]: no method named `dealloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\destructors.md:79:20
   |
26 |             Global.dealloc(self.my_box.ptr.as_ptr() as *mut _, Layout::new::<T>());
   |                    ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
4  | use std::alloc::Alloc;
   |
thread 'C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 54)' panicked at 'couldn't compile the test', librustdoc\test.rs:325:17
---- C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 125) stdout ----
error[E0599]: no method named `dealloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\destructors.md:138:20
   |
14 |             Global.dealloc(self.ptr.as_ptr() as *mut _, Layout::new::<T>());
   |                    ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
4  | use std::alloc::Alloc;
   |
error[E0599]: no method named `dealloc` found for type `std::alloc::Global` in the current scope
  --> C:\projects\rust\src/doc/nomicon\src\destructors.md:152:20
   |
28 |             Global.dealloc(my_box.ptr.as_ptr() as *mut _, Layout::new::<T>());
   |                    ^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
4  | use std::alloc::Alloc;
   |
thread 'C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 125)' panicked at 'couldn't compile the test', librustdoc\test.rs:325:17
failures:
    C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 125)
    C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 28)
    C:\projects\rust\src/doc/nomicon\src\destructors.md - Destructors (line 54)
test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
