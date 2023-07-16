plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0624]: associated function `reserve_exact` is private
    --> library/alloc/src/collections/vec_deque/mod.rs:715:22
     |
715  |             self.buf.reserve_exact(used_cap, new_cap - used_cap);
     |
    ::: /checkout/library/alloc/src/boxed.rs:1028:5
     |
     |
1028 |     pub(crate) fn reserve_exact(&mut self, len: usize, additional: usize) {


error[E0624]: associated function `try_reserve_exact` is private
     |
     |
806  |               self.buf.try_reserve_exact(used_cap, new_cap - used_cap)?;
     |
    ::: /checkout/library/alloc/src/boxed.rs:1033:5
     |
1033 | /     pub(crate) fn try_reserve_exact(
1033 | /     pub(crate) fn try_reserve_exact(
1034 | |         &mut self,
1035 | |         len: usize,
1036 | |         additional: usize,
1037 | |     ) -> Result<(), TryReserveError> {
     | |____________________________________- private associated function defined here

error[E0624]: associated function `shrink_to_fit` is private
    |
    |
910 |             self.buf.shrink_to_fit(target_cap);
    |
   ::: /checkout/library/alloc/src/boxed.rs:997:5
    |
    |
997 |     pub(crate) fn shrink_to_fit(&mut self, cap: usize) {

error[E0624]: associated function `reserve_exact` is private
    --> library/alloc/src/collections/vec_deque/mod.rs:2236:18
     |
     |
2236 |         self.buf.reserve_exact(old_cap, old_cap);
     |
    ::: /checkout/library/alloc/src/boxed.rs:1028:5
     |
     |
1028 |     pub(crate) fn reserve_exact(&mut self, len: usize, additional: usize) {

error[E0624]: associated function `from_raw_slice_parts_in` is private
    --> library/alloc/src/collections/vec_deque/mod.rs:3009:28
     |
     |
3009 |             let buf = Box::from_raw_slice_parts_in(other_buf.cast(), capacity, alloc);
     |
    ::: /checkout/library/alloc/src/boxed.rs:832:5
     |
     |
832  |     pub(crate) unsafe fn from_raw_slice_parts_in(ptr: *mut T, len: usize, alloc: A) -> Self {

error[E0624]: associated function `reserve` is private
   --> library/alloc/src/vec/splice.rs:123:17
    |
    |
123 |         vec.buf.reserve(len, additional);
    |
   ::: /checkout/library/alloc/src/boxed.rs:974:5
    |
    |
974 |     pub(crate) fn reserve(&mut self, len: usize, additional: usize) {

error[E0624]: associated function `from_raw_slice_parts_in` is private
   --> library/alloc/src/vec/into_iter.rs:312:34
    |
    |
312 |                     let _ = Box::from_raw_slice_parts_in(self.0.buf.as_ptr(), self.0.cap, alloc);
    |
   ::: /checkout/library/alloc/src/boxed.rs:832:5
    |
    |
832 |     pub(crate) unsafe fn from_raw_slice_parts_in(ptr: *mut T, len: usize, alloc: A) -> Self {


error[E0624]: associated constant `MIN_NON_ZERO_CAP` is private
    |
    |
32  |                       cmp::max(Box::<[T]>::MIN_NON_ZERO_CAP, lower.saturating_add(1));
    |
   ::: /checkout/library/alloc/src/boxed.rs:587:5
    |
    |
587 | /     pub(crate) const MIN_NON_ZERO_CAP: usize = if mem::size_of::<T>() == 1 {
588 | |         8
589 | |     } else if mem::size_of::<T>() <= 1024 {
591 | |     } else {
592 | |         1
593 | |     };
    | |______- private associated constant defined here
    | |______- private associated constant defined here

error[E0624]: associated constant `EMPTY` is private
    |
    |
424 |         Vec { buf: Box::<[MaybeUninit<T>]>::EMPTY, len: 0 }
    |
   ::: /checkout/library/alloc/src/boxed.rs:717:5
    |
717 |     pub(crate) const EMPTY: Self = Self::empty();
717 |     pub(crate) const EMPTY: Self = Self::empty();
    |     --------------------------------------------- private associated constant defined here

error[E0624]: associated function `from_raw_slice_parts_in` is private
   --> library/alloc/src/vec/mod.rs:685:29
    |
685 |             Vec { buf: Box::from_raw_slice_parts_in(ptr.cast(), capacity, alloc), len: length }
    |
   ::: /checkout/library/alloc/src/boxed.rs:832:5
    |
    |
832 |     pub(crate) unsafe fn from_raw_slice_parts_in(ptr: *mut T, len: usize, alloc: A) -> Self {

error[E0624]: associated function `reserve` is private
   --> library/alloc/src/vec/mod.rs:810:18
    |
    |
810 |         self.buf.reserve(self.len, additional);
    |
   ::: /checkout/library/alloc/src/boxed.rs:974:5
    |
    |
974 |     pub(crate) fn reserve(&mut self, len: usize, additional: usize) {

error[E0624]: associated function `reserve_exact` is private
    --> library/alloc/src/vec/mod.rs:838:18
     |
     |
838  |         self.buf.reserve_exact(self.len, additional);
     |
    ::: /checkout/library/alloc/src/boxed.rs:1028:5
     |
     |
1028 |     pub(crate) fn reserve_exact(&mut self, len: usize, additional: usize) {

error[E0624]: associated function `try_reserve` is private
    --> library/alloc/src/vec/mod.rs:874:18
     |
     |
874  |           self.buf.try_reserve(self.len, additional)
     |
    ::: /checkout/library/alloc/src/boxed.rs:1002:5
     |
1002 | /     pub(crate) fn try_reserve(
1002 | /     pub(crate) fn try_reserve(
1003 | |         &mut self,
1004 | |         len: usize,
1005 | |         additional: usize,
1006 | |     ) -> Result<(), TryReserveError> {
     | |____________________________________- private associated function defined here

error[E0624]: associated function `try_reserve_exact` is private
     |
     |
916  |           self.buf.try_reserve_exact(self.len, additional)
     |
    ::: /checkout/library/alloc/src/boxed.rs:1033:5
     |
1033 | /     pub(crate) fn try_reserve_exact(
1033 | /     pub(crate) fn try_reserve_exact(
1034 | |         &mut self,
1035 | |         len: usize,
1036 | |         additional: usize,
1037 | |     ) -> Result<(), TryReserveError> {
     | |____________________________________- private associated function defined here

error[E0624]: associated function `shrink_to_fit` is private
    |
    |
940 |             self.buf.shrink_to_fit(self.len);
    |
   ::: /checkout/library/alloc/src/boxed.rs:997:5
    |
    |
997 |     pub(crate) fn shrink_to_fit(&mut self, cap: usize) {


error[E0624]: associated function `shrink_to_fit` is private
    |
    |
966 |             self.buf.shrink_to_fit(cmp::max(self.len, min_capacity));
    |
   ::: /checkout/library/alloc/src/boxed.rs:997:5
    |
    |
997 |     pub(crate) fn shrink_to_fit(&mut self, cap: usize) {


error[E0624]: associated function `reserve_for_push` is private
     |
     |
1729 |             self.buf.reserve_for_push(self.len);
     |
    ::: /checkout/library/alloc/src/boxed.rs:982:5
     |
     |
982  |     pub(crate) fn reserve_for_push(&mut self, len: usize) {

For more information about this error, try `rustc --explain E0624`.
error: could not compile `alloc` due to 17 previous errors
warning: build failed, waiting for other jobs to finish...
