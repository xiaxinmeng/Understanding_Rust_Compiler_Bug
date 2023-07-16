plain
   |
14 |     drop(x);
   |     ^^^^^-^
   |          |
   |          argument has type `ManuallyDrop<TypeWithDrop>`
   |
   = note: `-D undropped-manually-drops` implied by `-D warnings`
help: use `std::mem::ManuallyDrop::into_inner` to get the inner value
   |
14 |     drop(std::mem::ManuallyDrop::into_inner(x));
   |          +++++++++++++++++++++++++++++++++++ +
error: calls to `std::mem::drop` with `std::mem::ManuallyDrop` instead of the inner value does nothing
  --> library/core/tests/manually_drop.rs:25:5
   |
25 |     drop(x);
25 |     drop(x);
   |     ^^^^^-^
   |          |
   |          argument has type `ManuallyDrop<TypeWithDrop>`
help: use `std::mem::ManuallyDrop::into_inner` to get the inner value
   |
   |
25 |     drop(std::mem::ManuallyDrop::into_inner(x));
   |          +++++++++++++++++++++++++++++++++++ +
error: calls to `std::mem::drop` with `std::mem::ManuallyDrop` instead of the inner value does nothing
  --> library/core/tests/manually_drop.rs:26:5
   |
26 |     drop(y);
26 |     drop(y);
   |     ^^^^^-^
   |          |
   |          argument has type `ManuallyDrop<TypeWithDrop>`
help: use `std::mem::ManuallyDrop::into_inner` to get the inner value
   |
   |
26 |     drop(std::mem::ManuallyDrop::into_inner(y));
   |          +++++++++++++++++++++++++++++++++++ +
error: could not compile `core` (test "coretests") due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:15:41
