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

error: associated function is never used: `from_box`
   --> library/alloc/src/raw_vec.rs:187:12
    |
    |
187 |     pub fn from_box(slice: Box<[T], A>) -> Self {

error: associated function is never used: `shrink`
   --> library/alloc/src/raw_vec.rs:513:8
    |
    |
513 |     fn shrink(&mut self, amount: usize) -> Result<(), TryReserveError> {

error: implementation has missing stability attribute
   --> library/alloc/src/collections/mod.rs:138:1
    |
    |
138 | / impl From<LayoutError> for TryReserveError {
139 | |     #[inline]
140 | |     fn from(_: LayoutError) -> Self {
141 | |         TryReserveError::from(TryReserveErrorKind::CapacityOverflow)
143 | | }
    | |_^

error: could not compile `alloc` due to 7 previous errors
