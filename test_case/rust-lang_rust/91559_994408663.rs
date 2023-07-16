plain
    Checking rustc-demangle v0.1.21
error: associated function is never used: `try_with_capacity`
   --> library/alloc/src/raw_vec.rs:102:12
    |
102 |     pub fn try_with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: associated function is never used: `try_with_capacity_zeroed`
   --> library/alloc/src/raw_vec.rs:116:12
    |
    |
116 |     pub fn try_with_capacity_zeroed(capacity: usize) -> Result<Self, TryReserveError> {

error: associated function is never used: `from_raw_parts`
   --> library/alloc/src/raw_vec.rs:129:19
    |
    |
129 |     pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize) -> Self {

error: associated function is never used: `try_with_capacity_zeroed_in`
   --> library/alloc/src/raw_vec.rs:182:12
    |
    |
182 |     pub fn try_with_capacity_zeroed_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {

error: associated function is never used: `shrink`
   --> library/alloc/src/raw_vec.rs:500:8
    |
    |
500 |     fn shrink(&mut self, amount: usize) -> Result<(), TryReserveError> {

error: could not compile `alloc` due to 5 previous errors
Build completed unsuccessfully in 0:01:32
