
error[E0311]: the parameter type `T` may not live long enough
  --> src/lib.rs:20:8
   |
14 | impl<T: ToString, const N: usize> WithStrings for [T; N] {
   |      -- help: consider adding an explicit lifetime bound...: `T: 'a +`
...
20 |     fn with_strings<R>(self, f: impl FnOnce(Self::Strings<'_>) -> R) -> R {
   |        ^^^^^^^^^^^^ ...so that the type `[T; N]` will meet its required lifetime bounds
