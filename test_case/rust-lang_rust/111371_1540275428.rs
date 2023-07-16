
error: struct has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:313:1
    |
313 | / struct FlattenCompat<I, U> {
314 | |     iter: Fuse<I>,
315 | |     frontiter: Option<U>,
316 | |     backiter: Option<U>,
317 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:312:10
    |
312 |   #[derive(Clone, Debug)]
    |            ^^^^^ in this derive macro expansion
    |
   ::: library/core/src/clone.rs:138:1
    |
138 | / pub macro Clone($item:item) {
139 | |     /* compiler built-in */
140 | | }
    | |_- in this expansion of `#[derive(Clone)]`

error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:312:17
    |
312 |   #[derive(Clone, Debug)]
    |                   ^^^^^ in this derive macro expansion
    |
   ::: library/core/src/fmt/mod.rs:588:5
    |
588 | /     pub macro Debug($item:item) {
589 | |         /* compiler built-in */
590 | |     }
    | |_____- in this expansion of `#[derive(Debug)]`
error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:466:1
    |
466 | / impl<I, U> Iterator for FlattenCompat<I, U>
467 | | where
468 | |     I: Iterator<Item: IntoIterator<IntoIter = U, Item = U::Item>>,
469 | |     U: Iterator,
...   |
577 | |     }
578 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:580:1
    |
580 | / impl<I, U> DoubleEndedIterator for FlattenCompat<I, U>
581 | | where
582 | |     I: DoubleEndedIterator<Item: IntoIterator<IntoIter = U, Item = U::Item>>,
583 | |     U: DoubleEndedIterator,
...   |
646 | |     }
647 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:649:1
    |
649 | / unsafe impl<const N: usize, I, T> TrustedLen
650 | |     for FlattenCompat<I, <[T; N] as IntoIterator>::IntoIter>
651 | | where
652 | |     I: TrustedLen<Item = [T; N]>,
653 | | {
654 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:656:1
    |
656 | / unsafe impl<'a, const N: usize, I, T> TrustedLen
657 | |     for FlattenCompat<I, <&'a [T; N] as IntoIterator>::IntoIter>
658 | | where
659 | |     I: TrustedLen<Item = &'a [T; N]>,
660 | | {
661 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/iter/adapters/flatten.rs:663:1
    |
663 | / unsafe impl<'a, const N: usize, I, T> TrustedLen
664 | |     for FlattenCompat<I, <&'a mut [T; N] as IntoIterator>::IntoIter>
665 | | where
666 | |     I: TrustedLen<Item = &'a mut [T; N]>,
667 | | {
668 | | }
    | |_^

error: could not compile `core` (lib) due to 8 previous errors
