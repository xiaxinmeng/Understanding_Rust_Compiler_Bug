plain
    Checking rustc-demangle v0.1.21
error[E0412]: cannot find type `MaybeUninit` in this scope
   --> library/alloc/src/boxed.rs:614:46
    |
614 |     pub const fn take(this: Self) -> (T, Box<MaybeUninit<T>, A>) {
    |
help: consider importing one of these items
    |
135 | use core::mem::MaybeUninit;
135 | use core::mem::MaybeUninit;
    |
135 | use crate::boxed::mem::MaybeUninit;
    |

error[E0425]: cannot find value `boxed` in this scope
   --> library/alloc/src/boxed.rs:615:57
    |
615 |         let (raw, alloc) = Box::into_raw_with_allocator(boxed);

error[E0412]: cannot find type `MaybeUninit` in this scope
   --> library/alloc/src/boxed.rs:626:55
    |
    |
626 |             let new_box = Box::from_raw_in(raw.cast::<MaybeUninit<T>>(), alloc);
    |
help: consider importing one of these items
    |
135 | use core::mem::MaybeUninit;
135 | use core::mem::MaybeUninit;
    |
135 | use crate::boxed::mem::MaybeUninit;
    |

error[E0699]: the type of this value must be known to call a method on a raw pointer on it
    |
    |
626 |             let new_box = Box::from_raw_in(raw.cast::<MaybeUninit<T>>(), alloc);

Some errors have detailed explanations: E0412, E0425, E0699.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `alloc` due to 4 previous errors
