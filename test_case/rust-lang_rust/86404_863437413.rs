plain
    Checking rustc-demangle v0.1.18
error[E0658]: use of unstable library feature 'array_len'
    --> library/alloc/src/collections/vec_deque/macros.rs:10:40
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<A, B, $($vars)*> PartialEq<$rhs> for $lhs
...    |
10   | |                 if self.len() != other.len() {
...    |
18   | |     }
19   | | }
19   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
    ::: library/alloc/src/collections/vec_deque/mod.rs:2685:1
     |
     |
2685 |   __impl_slice_eq1! { [const N: usize] VecDeque<A>, [B; N], }
     |
     = help: add `#![feature(array_len)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'array_len'
error[E0658]: use of unstable library feature 'array_len'
    --> library/alloc/src/collections/vec_deque/macros.rs:10:40
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<A, B, $($vars)*> PartialEq<$rhs> for $lhs
...    |
10   | |                 if self.len() != other.len() {
...    |
18   | |     }
19   | | }
19   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
    ::: library/alloc/src/collections/vec_deque/mod.rs:2686:1
     |
     |
2686 |   __impl_slice_eq1! { [const N: usize] VecDeque<A>, &[B; N], }
     |
     = help: add `#![feature(array_len)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'array_len'
error[E0658]: use of unstable library feature 'array_len'
    --> library/alloc/src/collections/vec_deque/macros.rs:10:40
     |
1    | / macro_rules! __impl_slice_eq1 {
2    | |     ([$($vars:tt)*] $lhs:ty, $rhs:ty, $($constraints:tt)*) => {
3    | |         #[stable(feature = "vec_deque_partial_eq_slice", since = "1.17.0")]
4    | |         impl<A, B, $($vars)*> PartialEq<$rhs> for $lhs
...    |
10   | |                 if self.len() != other.len() {
...    |
18   | |     }
19   | | }
19   | | }
     | |_- in this expansion of `__impl_slice_eq1!`
    ::: library/alloc/src/collections/vec_deque/mod.rs:2687:1
     |
     |
2687 |   __impl_slice_eq1! { [const N: usize] VecDeque<A>, &mut [B; N], }
     |
     = help: add `#![feature(array_len)]` to the crate attributes to enable

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:12
