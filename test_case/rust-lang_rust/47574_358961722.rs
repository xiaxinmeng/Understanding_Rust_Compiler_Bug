
error[E0119]: conflicting implementations of trait `iter_private::TrustedRandomAccess` for type `iter::Cloned<_>`:
   --> libcore\iter\mod.rs:593:1
    |
581 | / default unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
582 | |     where I: TrustedRandomAccess<Item=&'a T>, T: Clone
583 | | {
584 | |     unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
...   |
589 | |     fn may_have_side_effect() -> bool { true }
590 | | }
    | |_- first implementation here
...
593 | / unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
594 | |     where I: TrustedRandomAccess<Item=&'a T>, T: Copy
595 | | {
596 | |     unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
...   |
601 | |     fn may_have_side_effect() -> bool { false }
602 | | }
    | |_^ conflicting implementation for `iter::Cloned<_>`

error: aborting due to previous error

error: Could not compile `core`.
