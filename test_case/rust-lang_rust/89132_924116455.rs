plain
    Checking rustc-demangle v0.1.21
error[E0382]: use of moved value: `alloc`
   --> library/alloc/src/rc.rs:740:13
    |
729 |     pub fn try_new_in(value: T, alloc: A) -> Result<Self, AllocError> {
    |                                 ----- move occurs because `alloc` has type `A`, which does not implement the `Copy` trait
737 |                 alloc,
    |                 ----- value moved here
...
740 |             alloc,
740 |             alloc,
    |             ^^^^^ value used here after move

error[E0509]: cannot move out of type `Rc<MaybeUninit<T>, A>`, which implements the `Drop` trait
     |
     |
1003 |         Rc::from_inner_in(mem::ManuallyDrop::new(self).ptr.cast(), self.alloc)
     |                                                                    |
     |                                                                    cannot move out of here
     |                                                                    cannot move out of here
     |                                                                    move occurs because `self.alloc` has type `A`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `self`
    --> library/alloc/src/rc.rs:1003:68
     |
     |
1002 |     pub unsafe fn assume_init(self) -> Rc<T, A> {
     |                               ---- move occurs because `self` has type `Rc<MaybeUninit<T>, A>`, which does not implement the `Copy` trait
1003 |         Rc::from_inner_in(mem::ManuallyDrop::new(self).ptr.cast(), self.alloc)
     |                                                  ----              ^^^^^^^^^^ value used here after move
     |                                                  value moved here


error[E0509]: cannot move out of type `Rc<[MaybeUninit<T>], A>`, which implements the `Drop` trait
     |
     |
1044 |         unsafe { Rc::from_ptr_in(mem::ManuallyDrop::new(self).ptr.as_ptr() as _, self.alloc) }
     |                                                                                  |
     |                                                                                  cannot move out of here
     |                                                                                  cannot move out of here
     |                                                                                  move occurs because `self.alloc` has type `A`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `self`
    --> library/alloc/src/rc.rs:1044:82
     |
     |
1043 |     pub unsafe fn assume_init(self) -> Rc<[T], A> {
     |                               ---- move occurs because `self` has type `Rc<[MaybeUninit<T>], A>`, which does not implement the `Copy` trait
1044 |         unsafe { Rc::from_ptr_in(mem::ManuallyDrop::new(self).ptr.as_ptr() as _, self.alloc) }
     |                                                         ---- value moved here    ^^^^^^^^^^ value used here after move

error[E0509]: cannot move out of type `Rc<dyn Any, A>`, which implements the `Drop` trait
     |
     |
1610 |             Ok(Rc::from_inner_in(ptr, self.alloc))
     |                                       |
     |                                       cannot move out of here
     |                                       cannot move out of here
     |                                       move occurs because `self.alloc` has type `A`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `self`
    --> library/alloc/src/rc.rs:1610:39
     |
     |
1606 |     pub fn downcast<T: Any>(self) -> Result<Rc<T, A>, Self> {
     |                             ---- move occurs because `self` has type `Rc<dyn Any, A>`, which does not implement the `Copy` trait
1609 |             forget(self);
     |                    ---- value moved here
     |                    ---- value moved here
1610 |             Ok(Rc::from_inner_in(ptr, self.alloc))
     |                                       ^^^^^^^^^^ value used here after move
error[E0382]: use of moved value: `alloc`
    --> library/alloc/src/rc.rs:1743:36
     |
     |
1727 |             let (box_unique, alloc) = Box::into_unique(v);
     |                              ----- move occurs because `alloc` has type `A`, which does not implement the `Copy` trait
...
1741 |             box_free(box_unique, alloc);
     |                                  ----- value moved here
1742 | 
1743 |             Self::from_ptr_in(ptr, alloc)
     |                                    ^^^^^ value used here after move
error: variable does not need to be mutable
    --> library/alloc/src/rc.rs:2382:13
     |
     |
2382 |     fn from(mut v: Vec<T, A>) -> Rc<[T], A> {
     |             |
     |             help: remove this `mut`
     |
     |
     = note: `-D unused-mut` implied by `-D warnings`

error[E0509]: cannot move out of type `rc::Weak<T, A>`, which implements the `Drop` trait
     |
2783 |         let alloc = self.alloc;
     |                     ^^^^^^^^^^
     |                     |
     |                     |
     |                     cannot move out of here
     |                     move occurs because `self.alloc` has type `A`, which does not implement the `Copy` trait
     |                     help: consider borrowing here: `&self.alloc`
Some errors have detailed explanations: E0382, E0509.
For more information about an error, try `rustc --explain E0382`.
error: could not compile `alloc` due to 10 previous errors
Build completed unsuccessfully in 0:01:18
