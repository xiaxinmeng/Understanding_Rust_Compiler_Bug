plain
    Checking rustc-demangle v0.1.21
error[E0053]: method `partial_cmp` has an incompatible type for trait
    --> library/alloc/src/rc.rs:1952:34
     |
1936 | impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Rc<T, A> {
     |                              - this type parameter
...
1952 |     fn partial_cmp(&self, other: &Rc<T>) -> Option<Ordering> {
     |                                  |
     |                                  |
     |                                  expected type parameter `A`, found struct `Global`
     |                                  help: change the parameter type to match the trait: `&Rc<T, A>`
     |
     = note: expected fn pointer `fn(&Rc<T, A>, &Rc<T, A>) -> Option<_>`
                found fn pointer `fn(&Rc<T, A>, &Rc<T>) -> Option<_>`

error[E0053]: method `lt` has an incompatible type for trait
     |
     |
1936 | impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Rc<T, A> {
     |                              - this type parameter
...
1970 |     fn lt(&self, other: &Rc<T>) -> bool {
     |                         |
     |                         |
     |                         expected type parameter `A`, found struct `Global`
     |                         help: change the parameter type to match the trait: `&Rc<T, A>`
     |
     = note: expected fn pointer `fn(&Rc<T, A>, &Rc<T, A>) -> _`
                found fn pointer `fn(&Rc<T, A>, &Rc<T>) -> _`
error[E0053]: method `le` has an incompatible type for trait
    --> library/alloc/src/rc.rs:1988:25
     |
     |
1936 | impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Rc<T, A> {
     |                              - this type parameter
...
1988 |     fn le(&self, other: &Rc<T>) -> bool {
     |                         |
     |                         |
     |                         expected type parameter `A`, found struct `Global`
     |                         help: change the parameter type to match the trait: `&Rc<T, A>`
     |
     = note: expected fn pointer `fn(&Rc<T, A>, &Rc<T, A>) -> _`
                found fn pointer `fn(&Rc<T, A>, &Rc<T>) -> _`

error[E0053]: method `gt` has an incompatible type for trait
     |
     |
1936 | impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Rc<T, A> {
     |                              - this type parameter
...
2006 |     fn gt(&self, other: &Rc<T>) -> bool {
     |                         |
     |                         |
     |                         expected type parameter `A`, found struct `Global`
     |                         help: change the parameter type to match the trait: `&Rc<T, A>`
     |
     = note: expected fn pointer `fn(&Rc<T, A>, &Rc<T, A>) -> _`
                found fn pointer `fn(&Rc<T, A>, &Rc<T>) -> _`
error[E0053]: method `ge` has an incompatible type for trait
    --> library/alloc/src/rc.rs:2024:25
     |
     |
1936 | impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Rc<T, A> {
     |                              - this type parameter
...
2024 |     fn ge(&self, other: &Rc<T>) -> bool {
     |                         |
     |                         |
     |                         expected type parameter `A`, found struct `Global`
     |                         help: change the parameter type to match the trait: `&Rc<T, A>`
     |
     = note: expected fn pointer `fn(&Rc<T, A>, &Rc<T, A>) -> _`
                found fn pointer `fn(&Rc<T, A>, &Rc<T>) -> _`

error[E0053]: method `cmp` has an incompatible type for trait
     |
     |
2030 | impl<T: ?Sized + Ord, A: Allocator> Ord for Rc<T, A> {
     |                       - this type parameter
...
2046 |     fn cmp(&self, other: &Rc<T>) -> Ordering {
     |                          |
     |                          |
     |                          expected type parameter `A`, found struct `Global`
     |                          help: change the parameter type to match the trait: `&Rc<T, A>`
     |
     = note: expected fn pointer `fn(&Rc<T, A>, &Rc<T, A>) -> core::cmp::Ordering`
                found fn pointer `fn(&Rc<T, A>, &Rc<T>) -> core::cmp::Ordering`

error[E0599]: no function or associated item named `from_inner` found for struct `Rc<T, A>` in the current scope
    |
308 | / pub struct Rc<
308 | / pub struct Rc<
309 | |     T: ?Sized,
310 | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311 | | > {
314 | |     alloc: A,
315 | | }
315 | | }
    | |_- function or associated item `from_inner` not found for this
756 |           Ok(Self::from_inner(
    |                    ^^^^^^^^^^
    |                    |
    |                    |
    |                    function or associated item not found in `Rc<T, A>`
    |                    help: there is an associated function with a similar name: `from_inner_in`
    = note: the function or associated item was found for
    = note: the function or associated item was found for
            - `Rc<T>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1224:38
     |
     |
1109 | impl<T: ?Sized, A: Allocator> Rc<T, A> {
     |                 - this type parameter
...
1224 |         Weak { ptr: this.ptr, alloc: Global }
     |                                      ^^^^^^ expected type parameter `A`, found struct `Global`
     = note: expected type parameter `A`
                        found struct `Global`

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1291:76
     |
1109 | impl<T: ?Sized, A: Allocator> Rc<T, A> {
     |                 - this type parameter
...
1291 |         let rc = unsafe { mem::ManuallyDrop::new(Rc::<T>::from_raw_in(ptr, alloc)) };
     |                                                                            ^^^^^ expected struct `Global`, found type parameter `A`
     = note:      expected struct `Global`
             found type parameter `A`


error[E0599]: no function or associated item named `new_uninit` found for struct `Rc<T, A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `new_uninit` not found for this
1488 |               let mut rc = Self::new_uninit();
     |                                  ^^^^^^^^^^
     |                                  |
     |                                  |
     |                                  function or associated item not found in `Rc<T, A>`
     |                                  help: there is an associated function with a similar name: `new_uninit_in`
     = note: the function or associated item was found for
     = note: the function or associated item was found for
             - `Rc<T>`

error[E0599]: no function or associated item named `new_uninit` found for struct `Rc<T, A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `new_uninit` not found for this
1496 |               let mut rc = Self::new_uninit();
     |                                  ^^^^^^^^^^
     |                                  |
     |                                  |
     |                                  function or associated item not found in `Rc<T, A>`
     |                                  help: there is an associated function with a similar name: `new_uninit_in`
     = note: the function or associated item was found for
     = note: the function or associated item was found for
             - `Rc<T>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1542:16
     |
     |
1517 | impl<A: Allocator> Rc<dyn Any, A> {
     |      - this type parameter
...
1542 |             Ok(Rc::from_inner(ptr))
     |                ^^^^^^^^^^^^^^^^^^^ expected type parameter `A`, found struct `Global`
     |
     = note: expected struct `Rc<_, A>`
                found struct `Rc<_, Global>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1637:36
     |
     |
1619 |     fn from_box<A: Allocator>(v: Box<T, A>) -> Rc<T, A> {
     |                 - this type parameter
...
1637 |             Self::from_ptr_in(ptr, alloc)
     |                                    ^^^^^ expected struct `Global`, found type parameter `A`
     = note:      expected struct `Global`
             found type parameter `A`

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1637:13
     |
1619 |     fn from_box<A: Allocator>(v: Box<T, A>) -> Rc<T, A> {
     |                 - this type parameter          -------- expected `Rc<T, A>` because of return type
...
1637 |             Self::from_ptr_in(ptr, alloc)
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `A`, found struct `Global`
     |
     = note: expected struct `Rc<_, A>`
                found struct `Rc<_, Global>`

error[E0599]: no function or associated item named `from_iter_exact` found for struct `Rc<[T], A>` in the current scope
     |
308  | / pub struct Rc<
308  | / pub struct Rc<
309  | |     T: ?Sized,
310  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
311  | | > {
314  | |     alloc: A,
315  | | }
315  | | }
     | |_- function or associated item `from_iter_exact` not found for this
...
1726 |           unsafe { Self::from_iter_exact(v.iter().cloned(), v.len()) }
     |                          ^^^^^^^^^^^^^^^ function or associated item not found in `Rc<[T], A>`
     = note: the function or associated item was found for
     = note: the function or associated item was found for
             - `Rc<[T]>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1734:18
     |
     |
1731 | impl<T: Copy, A: Allocator> RcFromSlice<T> for Rc<[T], A> {
     |               - this type parameter
1732 |     #[inline]
1733 |     fn from_slice(v: &[T]) -> Self {
     |                               ---- expected `Rc<[T], A>` because of return type
1734 |         unsafe { Rc::copy_from_slice(v) }
     |                  ^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `A`, found struct `Global`
     |
     = note: expected struct `Rc<_, A>`
                found struct `Rc<_, Global>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:2619:68
     |
     |
2484 | impl<T: ?Sized, A: Allocator> Weak<T, A> {
     |                 - this type parameter
...
2619 |         Weak { ptr: unsafe { NonNull::new_unchecked(ptr) }, alloc: Global }
     |                                                                    ^^^^^^ expected type parameter `A`, found struct `Global`
     = note: expected type parameter `A`
                        found struct `Global`

Some errors have detailed explanations: E0053, E0308, E0599.
