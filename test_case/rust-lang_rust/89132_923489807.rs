plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0434]: can't capture dynamic environment in a fn item
     |
     |
1806 |                     alloc.deallocate(self.mem, self.layout);
     |
     |
     = help: use the `|| { ... }` closure form instead
error[E0424]: expected value, found module `self`
    --> library/alloc/src/rc.rs:1225:38
     |
     |
1221 |     pub fn downgrade(this: &Self) -> Weak<T, A> {
     |            --------- this function doesn't have a `self` parameter
...
1225 |         Weak { ptr: this.ptr, alloc: self.alloc.clone() }
     |                                      ^^^^ `self` value is a keyword only available in methods with a `self` parameter
     |
help: add a `self` receiver parameter to make the associated `fn` a method
     |
1221 |     pub fn downgrade(&self, this: &Self) -> Weak<T, A> {


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
1294 |           let _rc_clone: mem::ManuallyDrop<_> = rc.clone();
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

error[E0599]: no method named `clone` found for type parameter `A` in the current scope
     |
     |
1489 |             let mut rc = Self::new_uninit_in(this.alloc.clone());
     |                                                         ^^^^^ method not found in `A`
     = help: items from traits can only be used if the type parameter is bounded by the trait
     = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `clone`, perhaps you need to restrict type parameter `A` with it:
     |
1431 | impl<T: Clone, A: Clone + Allocator> Rc<T, A> {


error[E0599]: no method named `clone` found for type parameter `A` in the current scope
     |
     |
1497 |             let mut rc = Self::new_uninit_in(this.alloc.clone());
     |                                                         ^^^^^ method not found in `A`
     = help: items from traits can only be used if the type parameter is bounded by the trait
     = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `clone`, perhaps you need to restrict type parameter `A` with it:
     |
1431 | impl<T: Clone, A: Clone + Allocator> Rc<T, A> {

error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1638:36
     |
     |
1620 |     fn from_box<A: Allocator>(v: Box<T, A>) -> Rc<T, A> {
     |                 - this type parameter
...
1638 |             Self::from_ptr_in(ptr, alloc)
     |                                    ^^^^^ expected struct `Global`, found type parameter `A`
     = note:      expected struct `Global`
             found type parameter `A`

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1638:13
     |
1620 |     fn from_box<A: Allocator>(v: Box<T, A>) -> Rc<T, A> {
     |                 - this type parameter          -------- expected `Rc<T, A>` because of return type
...
1638 |             Self::from_ptr_in(ptr, alloc)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `A`, found struct `Global`
     |
     = note: expected struct `Rc<_, A>`
                found struct `Rc<_, Global>`

error[E0599]: no function or associated item named `allocate_for_layout` found for struct `Rc<T, A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_layout` not found for this
1649 |               Self::allocate_for_layout(
     |                     ^^^^^^^^^^^^^^^^^^^
     |                     |
     |                     |
     |                     function or associated item not found in `Rc<T, A>`
     |                     help: there is an associated function with a similar name: `allocate_for_ptr_in`
     = note: the function or associated item was found for
     = note: the function or associated item was found for
             - `Rc<T>`

error[E0599]: no function or associated item named `allocate_for_ptr` found for struct `Rc<T, A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_ptr` not found for this
...
1664 |               let ptr = Self::allocate_for_ptr(bptr, &alloc);
     |                               |
     |                               |
     |                               function or associated item not found in `Rc<T, A>`
     |                               help: there is an associated function with a similar name: `allocate_for_ptr_in`
     = note: the function or associated item was found for
     = note: the function or associated item was found for
             - `Rc<T>`

error[E0599]: no function or associated item named `allocate_for_layout` found for struct `Rc<[T], A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_layout` not found for this
1761 |               Self::allocate_for_layout(
     |                     ^^^^^^^^^^^^^^^^^^^
     |                     |
     |                     function or associated item not found in `Rc<[T], A>`
     |                     function or associated item not found in `Rc<[T], A>`
     |                     help: there is an associated function with a similar name: `allocate_for_ptr_in`
     |
     = note: the function or associated item was found for
             - `Rc<T>`

error[E0599]: no function or associated item named `allocate_for_slice` found for struct `Rc<[T], A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_slice` not found for this
...
1775 |               let ptr = Self::allocate_for_slice(v.len());
     |                               |
     |                               function or associated item not found in `Rc<[T], A>`
     |                               help: there is an associated function with a similar name: `allocate_for_slice_in`
     |
     |
     = note: the function or associated item was found for
             - `Rc<[T]>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1777:13
     |
     |
1756 | impl<T, A: Allocator> Rc<[T], A> {
     |         - this type parameter
...
1773 |     unsafe fn copy_from_slice_in(v: &[T], alloc: A) -> Rc<[T]> {
     |                                                        ------- expected `Rc<[T]>` because of return type
...
1777 |             Self::from_ptr_in(ptr, alloc)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Global`, found type parameter `A`
     |
     = note: expected struct `Rc<_, Global>`
                found struct `Rc<_, A>`

error[E0599]: no function or associated item named `allocate_for_slice` found for struct `Rc<[T], A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `allocate_for_slice` not found for this
1812 |               let ptr = Self::allocate_for_slice(len);
     |                               ^^^^^^^^^^^^^^^^^^
     |                               |
     |                               function or associated item not found in `Rc<[T], A>`
     |                               function or associated item not found in `Rc<[T], A>`
     |                               help: there is an associated function with a similar name: `allocate_for_slice_in`
     |
     = note: the function or associated item was found for
             - `Rc<[T]>`

error[E0599]: no function or associated item named `from_ptr` found for struct `Rc<[T], A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `from_ptr` not found for this
1830 |               Self::from_ptr(ptr)
     |                     ^^^^^^^^ function or associated item not found in `Rc<[T], A>`
     |
     = note: the function or associated item was found for
     = note: the function or associated item was found for
             - `Rc<T>`
Some errors have detailed explanations: E0308, E0424, E0434, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 14 previous errors
warning: build failed, waiting for other jobs to finish...
