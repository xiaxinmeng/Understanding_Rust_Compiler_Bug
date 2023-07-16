plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: no rules expected the token `#`
    |
    |
401 |     /// Returns a slice which contains items not yet handled by split.
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no rules expected this token in macro call
   ::: library/core/src/slice/iter/macros.rs:400:1
    |
400 | macro_rules! split_iter {
    | ----------------------- when calling this macro
    | ----------------------- when calling this macro

error[E0432]: unresolved import `iter::Split`
  --> library/core/src/slice/mod.rs:45:37
   |
45 | pub use iter::{RSplitN, RSplitNMut, Split, SplitMut, SplitN, SplitNMut};
   |                                     |
   |                                     |
   |                                     no `Split` in `slice::iter`
   |                                     help: a similar name exists in the module: `SplitN`
error[E0412]: cannot find type `Split` in this scope
   --> library/core/src/slice/iter.rs:568:32
    |
    |
568 |       pub struct RSplit { inner: Split }: Clone
    |
   ::: library/core/src/slice/iter/macros.rs:665:9
    |
    |
665 | /         $vis struct $rev<'a, T: 'a, P>
666 | |         where
667 | |             P: FnMut(&T) -> bool,
668 | |         {
669 | |             inner: $inner<'a, T, P>,
670 | |         }
    | |_________- similarly named struct `RSplit` defined here
help: a struct with a similar name exists
    |
    |
568 |     pub struct RSplit { inner: RSplit }: Clone
help: consider importing this struct
    |
6   | use crate::str::Split;
    |
    |

error[E0433]: failed to resolve: use of undeclared type `Split`
   --> library/core/src/slice/iter.rs:568:32
    |
568 |       pub struct RSplit { inner: Split }: Clone
    |
   ::: library/core/src/slice/iter/macros.rs:665:9
    |
    |
665 | /         $vis struct $rev<'a, T: 'a, P>
666 | |         where
667 | |             P: FnMut(&T) -> bool,
668 | |         {
669 | |             inner: $inner<'a, T, P>,
670 | |         }
    | |_________- similarly named struct `RSplit` defined here
help: a struct with a similar name exists
    |
    |
568 |     pub struct RSplit { inner: RSplit }: Clone
help: consider importing this struct
    |
6   | use crate::str::Split;
    |
    |

error[E0412]: cannot find type `Split` in this scope
   --> library/core/src/slice/iter.rs:738:33
    |
738 |       pub struct SplitN { inner:  Split }: Clone
    |
   ::: library/core/src/slice/iter/macros.rs:665:9
    |
    |
665 | /         $vis struct $rev<'a, T: 'a, P>
666 | |         where
667 | |             P: FnMut(&T) -> bool,
668 | |         {
669 | |             inner: $inner<'a, T, P>,
670 | |         }
    | |_________- similarly named struct `RSplit` defined here
help: a struct with a similar name exists
    |
    |
738 |     pub struct SplitN { inner:  RSplit }: Clone
help: consider importing this struct
    |
6   | use crate::str::Split;
    |
    |

error[E0392]: parameter `'a` is never used
   --> library/core/src/slice/iter/macros.rs:763:28
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
763 | |         pub struct $iter_n<'a, T: 'a, P>
    | |                            ^^ unused parameter
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    |
    |
   ::: library/core/src/slice/iter.rs:720:1
    |
720 | / iter_n! {
721 | |     #[stable(feature = "rust1", since = "1.0.0")]
722 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
723 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
741 | |     fn max_items;
742 | | }
    | |_- in this macro invocation
    |
    |
    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
   --> library/core/src/slice/iter/macros.rs:763:32
    |
751 | / macro_rules! iter_n {
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
763 | |         pub struct $iter_n<'a, T: 'a, P>
    | |                                ^ unused parameter
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    |
    |
   ::: library/core/src/slice/iter.rs:720:1
    |
720 | / iter_n! {
721 | |     #[stable(feature = "rust1", since = "1.0.0")]
722 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
723 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
741 | |     fn max_items;
742 | | }
    | |_- in this macro invocation
    |
    |
    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `P` is never used
   --> library/core/src/slice/iter/macros.rs:763:39
    |
751 | / macro_rules! iter_n {
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
763 | |         pub struct $iter_n<'a, T: 'a, P>
    | |                                       ^ unused parameter
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    |
    |
   ::: library/core/src/slice/iter.rs:720:1
    |
720 | / iter_n! {
721 | |     #[stable(feature = "rust1", since = "1.0.0")]
722 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
723 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
741 | |     fn max_items;
742 | | }
    | |_- in this macro invocation
    |
    |
    = help: consider removing `P`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `'a` is never used
   --> library/core/src/slice/iter/macros.rs:763:28
    |
751 | /  macro_rules! iter_n {
751 | /  macro_rules! iter_n {
752 | |      (
753 | |          #[$stability:meta]
754 | |          #[fused($fused_stability:meta)]
...   |
763 | |          pub struct $iter_n<'a, T: 'a, P>
    | |                             ^^ unused parameter
827 | |      };
828 | |  }
    | |__- in this expansion of `iter_n!`
    |
    |
   ::: library/core/src/slice/iter.rs:744:1
    |
744 |  / iter_n! {
745 |  |     #[stable(feature = "rust1", since = "1.0.0")]
746 |  |     #[fused(stable(feature = "fused", since = "1.26.0"))]
747 |  |     #[must_use = "iterators are lazy and do nothing unless consumed"]
766 |  |     fn max_items;
767 |  | }
    |  |_- in this macro invocation
    |
    |
    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
   --> library/core/src/slice/iter/macros.rs:763:32
    |
751 | /  macro_rules! iter_n {
751 | /  macro_rules! iter_n {
752 | |      (
753 | |          #[$stability:meta]
754 | |          #[fused($fused_stability:meta)]
...   |
763 | |          pub struct $iter_n<'a, T: 'a, P>
    | |                                 ^ unused parameter
827 | |      };
828 | |  }
    | |__- in this expansion of `iter_n!`
    |
    |
   ::: library/core/src/slice/iter.rs:744:1
    |
744 |  / iter_n! {
745 |  |     #[stable(feature = "rust1", since = "1.0.0")]
746 |  |     #[fused(stable(feature = "fused", since = "1.26.0"))]
747 |  |     #[must_use = "iterators are lazy and do nothing unless consumed"]
766 |  |     fn max_items;
767 |  | }
    |  |_- in this macro invocation
    |
    |
    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `P` is never used
   --> library/core/src/slice/iter/macros.rs:763:39
    |
751 | /  macro_rules! iter_n {
751 | /  macro_rules! iter_n {
752 | |      (
753 | |          #[$stability:meta]
754 | |          #[fused($fused_stability:meta)]
...   |
763 | |          pub struct $iter_n<'a, T: 'a, P>
    | |                                        ^ unused parameter
827 | |      };
828 | |  }
    | |__- in this expansion of `iter_n!`
    |
    |
   ::: library/core/src/slice/iter.rs:744:1
    |
744 |  / iter_n! {
745 |  |     #[stable(feature = "rust1", since = "1.0.0")]
746 |  |     #[fused(stable(feature = "fused", since = "1.26.0"))]
747 |  |     #[must_use = "iterators are lazy and do nothing unless consumed"]
766 |  |     fn max_items;
767 |  | }
    |  |_- in this macro invocation
    |
    |
    = help: consider removing `P`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `'a` is never used
    --> library/core/src/str/iter.rs:1630:33
     |
     |
1630 | pub struct SplitAsciiWhitespace<'a> {
     |                                 ^^ unused parameter
     |
     = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
Some errors have detailed explanations: E0392, E0412, E0432, E0433.
For more information about an error, try `rustc --explain E0392`.
error: could not compile `core` due to 12 previous errors
warning: build failed, waiting for other jobs to finish...
