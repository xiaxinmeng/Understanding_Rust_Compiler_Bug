plain
   Compiling rustc-demangle v0.1.21
error[E0401]: can't use generic parameters from outer function
    --> library/alloc/src/rc.rs:1803:24
     |
1760 | impl<T, A: Allocator> Rc<[T], A> {
     |         - type parameter from outer function
1790 |     unsafe fn from_iter_exact_in(
1790 |     unsafe fn from_iter_exact_in(
     |               ------------------ try adding a local generic parameter in this method instead
1803 |             alloc: &'a A,
     |                        ^ use of generic parameter from outer function

error[E0424]: expected value, found module `self`
error[E0424]: expected value, found module `self`
    --> library/alloc/src/rc.rs:1228:38
     |
1221 |     pub fn downgrade(this: &Self) -> Weak<T, A>
     |            --------- this function doesn't have a `self` parameter
...
1228 |         Weak { ptr: this.ptr, alloc: self.alloc.clone() }
     |                                      ^^^^ `self` value is a keyword only available in methods with a `self` parameter
     |
help: add a `self` receiver parameter to make the associated `fn` a method
     |
1221 |     pub fn downgrade(&self, this: &Self) -> Weak<T, A>

error[E0425]: cannot find value `alloc` in this scope
    --> library/alloc/src/rc.rs:1639:34
     |
     |
1639 |             box_free(box_unique, alloc);
     |                                  ^^^^^ a field by this name exists in `Self`
help: consider importing this function
     |
246  | use crate::alloc::alloc;
     |
     |

error[E0726]: implicit elided lifetime not allowed here
     |
     |
1806 |         impl<T> Drop for Guard<T> {
     |                          ^^^^^^^^ help: indicate the anonymous lifetime: `Guard<'_, T>`
     |
     = note: assuming a `'static` lifetime...
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1806:9
     |
     |
1806 | /         impl<T> Drop for Guard<T> {
1807 | |             fn drop(&mut self) {
1808 | |                 unsafe {
1809 | |                     let slice = from_raw_parts_mut(self.elems, self.n_elems);
1814 | |             }
1815 | |         }
     | |_________^ lifetime mismatch
     |
     |
     = note: expected struct `Rc<[T], A>::from_iter_exact_in::Guard<'a, _>`
                found struct `Rc<[T], A>::from_iter_exact_in::Guard<'static, _>`
note: the lifetime `'a` as defined on the struct at 1798:22...
     |
     |
1798 |         struct Guard<'a, T> {
     |                      ^^
     = note: ...does not necessarily outlive the static lifetime

error[E0599]: the method `clone` exists for struct `ManuallyDrop<Rc<T, A>>`, but its trait bounds were not satisfied
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- doesn't satisfy `Rc<T, A>: Clone`
...
1297 |           let _rc_clone: mem::ManuallyDrop<_> = rc.clone();
     |                                                    ^^^^^ method cannot be called on `ManuallyDrop<Rc<T, A>>` due to unsatisfied trait bounds
    ::: /checkout/library/core/src/mem/manually_drop.rs:49:1
     |
     |
49   |   pub struct ManuallyDrop<T: ?Sized> {
     |   ---------------------------------- doesn't satisfy `ManuallyDrop<Rc<T, A>>: Clone`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Rc<T, A>: Clone`
             which is required by `ManuallyDrop<Rc<T, A>>: Clone`
             `A: Clone`
             which is required by `Rc<T, A>: Clone`
help: consider restricting the type parameter to satisfy the trait bound
     |
311  | > where A: Clone {
     |   ++++++++++++++

error[E0599]: no function or associated item named `allocate_for_layout_in` found for struct `Rc<T, A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_layout_in` not found for this
1652 |               Self::allocate_for_layout_in(
     |                     ^^^^^^^^^^^^^^^^^^^^^^
     |                     |
     |                     |
     |                     function or associated item not found in `Rc<T, A>`
     |                     help: there is an associated function with a similar name: `allocate_for_ptr_in`

error[E0599]: no function or associated item named `allocate_for_layout_in` found for struct `Rc<[T], A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_layout_in` not found for this
1765 |               Self::allocate_for_layout_in(
     |                     ^^^^^^^^^^^^^^^^^^^^^^
     |                     |
     |                     function or associated item not found in `Rc<[T], A>`
     |                     function or associated item not found in `Rc<[T], A>`
     |                     help: there is an associated function with a similar name: `allocate_for_ptr_in`

error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1782:13
     |
1760 | impl<T, A: Allocator> Rc<[T], A> {
     |         - this type parameter
...
1778 |     unsafe fn copy_from_slice_in(v: &[T], alloc: A) -> Rc<[T]> {
     |                                                        ------- expected `Rc<[T]>` because of return type
...
1782 |             Self::from_ptr_in(ptr, alloc)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Global`, found type parameter `A`
     |
     = note: expected struct `Rc<_, Global>`
                found struct `Rc<_, A>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1842:13
     |
     |
1760 | impl<T, A: Allocator> Rc<[T], A> {
     |         - this type parameter
...
1794 |     ) -> Rc<[T]> {
     |          ------- expected `Rc<[T]>` because of return type
...
1842 |             Self::from_ptr_in(ptr, alloc)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Global`, found type parameter `A`
     |
     = note: expected struct `Rc<_, Global>`
                found struct `Rc<_, A>`
Some errors have detailed explanations: E0308, E0401, E0424, E0425, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 10 previous errors
warning: build failed, waiting for other jobs to finish...
