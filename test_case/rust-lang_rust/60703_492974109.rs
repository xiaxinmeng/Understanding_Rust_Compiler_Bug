
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized] target(s) in 0.26s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling alloc v0.0.0 (/home/philipp/Documents/rust/src/liballoc)
error: unused import: `stage0_phantom`==================>              ] 29/38
  --> src/liballoc/sync.rs:27:60
   |
27 | use crate::alloc::{Global, Alloc, Layout, box_free_worker, stage0_phantom};
   |                                                            ^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: use of item 'core::mem::uninitialized' that will be deprecated in future version 2.0.0: use `mem::MaybeUninit::uninit` instead
   --> src/liballoc/boxed.rs:333:25
    |
333 |             let mut a = mem::uninitialized();
    |                         ^^^^^^^^^^^^^^^^^^
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of item 'core::mem::uninitialized' that will be deprecated in future version 2.0.0: use `mem::MaybeUninit::uninit` instead
   --> src/liballoc/sync.rs:302:29
    |
302 |             alloc: unsafe { mem::uninitialized() },
    |                             ^^^^^^^^^^^^^^^^^^

error: use of item 'core::mem::uninitialized' that will be deprecated in future version 2.0.0: use `mem::MaybeUninit::uninit` instead
   --> src/liballoc/rc.rs:319:29
    |
319 |             alloc: unsafe { mem::uninitialized() },
    |                             ^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src/liballoc/alloc.rs:243:26
    |
243 |         stage0_unphantom(a).dealloc(NonNull::from(ptr).cast(), layout);
    |                          ^
    |                          |
    |                          expected struct `core::marker::PhantomData`, found mutable reference
    |                          help: consider dereferencing the borrow: `*a`
    |
    = note: expected type `core::marker::PhantomData<_>`
               found type `&mut core::marker::PhantomData<A>`

error[E0277]: the trait bound `A: core::default::Default` is not satisfied
   --> src/liballoc/boxed.rs:247:47
    |
247 |         Box::from_raw_in(f(Box::into_raw(b)), stage0_unphantom(a))
    |                                               ^^^^^^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `A`
    |
    = help: consider adding a `where A: core::default::Default` bound
note: required by `alloc::stage0_unphantom`
   --> src/liballoc/alloc.rs:30:1
    |
30  | pub(crate) fn stage0_unphantom<A: Default>(_a: PhantomData<A>) -> A { A::default() }
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src/liballoc/boxed.rs:338:18
    |
338 |         (unique, alloc)
    |                  ^^^^^ expected type parameter, found struct `core::marker::PhantomData`
    |
    = note: expected type `A`
               found type `core::marker::PhantomData<A>`

error[E0277]: the trait bound `core::marker::PhantomData<A>: core::alloc::AllocHelper` is not satisfied
   --> src/liballoc/boxed.rs:917:21
    |
917 |         let Ok(b) = RawVec::with_capacity_in(self.len(), self.1.clone());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `core::alloc::AllocHelper` is not implemented for `core::marker::PhantomData<A>`

error[E0277]: the trait bound `core::marker::PhantomData<A>: core::alloc::AllocHelper` is not satisfied
   --> src/liballoc/boxed.rs:917:21
    |
917 |         let Ok(b) = RawVec::with_capacity_in(self.len(), self.1.clone());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `core::alloc::AllocHelper` is not implemented for `core::marker::PhantomData<A>`
    |
    = note: required because of the requirements on the impl of `core::alloc::Alloc` for `core::marker::PhantomData<A>`
note: required by `raw_vec::RawVec::<T, A>::with_capacity_in`
   --> src/liballoc/raw_vec.rs:68:5
    |
68  |     pub fn with_capacity_in(cap: usize, a: A) -> Result<Self, A::Err> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `core::marker::PhantomData<A>: core::alloc::AllocHelper` is not satisfied
   --> src/liballoc/boxed.rs:919:23
    |
919 |         let mut new = BoxBuilder {
    |                       ^^^^^^^^^^ the trait `core::alloc::AllocHelper` is not implemented for `core::marker::PhantomData<A>`
    |
    = note: required because of the requirements on the impl of `core::alloc::Alloc` for `core::marker::PhantomData<A>`
note: required by `<boxed::Box<[T], A> as core::clone::Clone>::clone::BoxBuilder`
   --> src/liballoc/boxed.rs:938:9
    |
938 |         struct BoxBuilder<T, A: Alloc> {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `ptr` found for type `raw_vec::RawVec<_, core::marker::PhantomData<A>>` in the current scope
   --> src/liballoc/boxed.rs:924:35
    |
924 |         let mut target = new.data.ptr();
    |                                   ^^^ private field, not a method
    | 
   ::: src/liballoc/raw_vec.rs:42:1
    |
42  | pub struct RawVec<T, A: Alloc = AbortAdapter<Global>> {
    | ----------------------------------------------------- method `ptr` not found for this
    |
    = note: the method `ptr` exists but the following trait bounds were not satisfied:
            `core::marker::PhantomData<A> : alloc::Stage0Alloc`

error[E0599]: no method named `into_box` found for type `<boxed::Box<[T], A> as core::clone::Clone>::clone::BoxBuilder<_, core::marker::PhantomData<A>>` in the current scope
   --> src/liballoc/boxed.rs:935:29
    |
935 |         return unsafe { new.into_box() };
    |                             ^^^^^^^^
...
938 |         struct BoxBuilder<T, A: Alloc> {
    |         ------------------------------ method `into_box` not found for this
    |
    = note: the method `into_box` exists but the following trait bounds were not satisfied:
            `core::marker::PhantomData<A> : alloc::Stage0Alloc`

error[E0277]: the trait bound `A: core::default::Default` is not satisfied
   --> src/liballoc/collections/linked_list.rs:675:30
    |
675 |         self.push_front_node(Box::new_in(Node::new(elt), alloc)?);
    |                              ^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `A`
    |
    = help: consider adding a `where A: core::default::Default` bound
note: required by `boxed::Box::<T, A>::new_in`
   --> src/liballoc/boxed.rs:155:5
    |
155 |     pub fn new_in(x: T, a: A) -> Result<Box<T, A>, A::Err> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `A: core::default::Default` is not satisfied
   --> src/liballoc/collections/linked_list.rs:720:29
    |
720 |         self.push_back_node(Box::new_in(Node::new(elt), alloc)?);
    |                             ^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `A`
    |
    = help: consider adding a `where A: core::default::Default` bound
note: required by `boxed::Box::<T, A>::new_in`
   --> src/liballoc/boxed.rs:155:5
    |
155 |     pub fn new_in(x: T, a: A) -> Result<Box<T, A>, A::Err> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `A: core::default::Default` is not satisfied
    --> src/liballoc/collections/linked_list.rs:1023:64
     |
1023 |                 let node = Some(NonNull::from(Box::into_unique(Box::new_in(Node {
     |                                                                ^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `A`
     |
     = help: consider adding a `where A: core::default::Default` bound
note: required by `boxed::Box::<T, A>::new_in`
    --> src/liballoc/boxed.rs:155:5
     |
155  |     pub fn new_in(x: T, a: A) -> Result<Box<T, A>, A::Err> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `A: core::default::Default` is not satisfied
   --> src/liballoc/sync.rs:299:28
    |
299 |         let x: Box<_, A> = Box::new_in(ArcInner {
    |                            ^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `A`
    |
    = help: consider adding a `where A: core::default::Default` bound
note: required by `boxed::Box::<T, A>::new_in`
   --> src/liballoc/boxed.rs:155:5
    |
155 |     pub fn new_in(x: T, a: A) -> Result<Box<T, A>, A::Err> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src/liballoc/sync.rs:668:32
    |
668 |             box_free_worker(u, &mut a);
    |                                ^^^^^^ expected struct `core::marker::PhantomData`, found type parameter
    |
    = note: expected type `&mut core::marker::PhantomData<_>`
               found type `&mut A2`

error[E0308]: mismatched types
   --> src/liballoc/sync.rs:679:32
    |
679 |             box_free_worker(u, &mut (*arc.ptr.as_ptr()).alloc);
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `core::marker::PhantomData`, found type parameter
    |
    = note: expected type `&mut core::marker::PhantomData<_>`
               found type `&mut A`

error[E0277]: the trait bound `A: core::default::Default` is not satisfied
   --> src/liballoc/rc.rs:316:28
    |
316 |         let x: Box<_, A> = Box::new_in(RcBox {
    |                            ^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `A`
    |
    = help: consider adding a `where A: core::default::Default` bound
note: required by `boxed::Box::<T, A>::new_in`
   --> src/liballoc/boxed.rs:155:5
    |
155 |     pub fn new_in(x: T, a: A) -> Result<Box<T, A>, A::Err> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src/liballoc/rc.rs:768:32
    |
768 |             box_free_worker(u, &mut (*rc.ptr.as_ptr()).alloc);
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `core::marker::PhantomData`, found type parameter
    |
    = note: expected type `&mut core::marker::PhantomData<_>`
               found type `&mut A`

error: aborting due to 20 previous errors

Some errors occurred: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: Could not compile `alloc`.
