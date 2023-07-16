plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:39:35
   |
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected enum `printf::Substitution`, found fn item
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs:16:5
   |
16 |       assert_eq!(pns("*so* has a %% escape"), Some((S::Escape, " escape")));
   |
   |
   = note: expected enum `Option<(printf::Substitution<'_>, &str)>`
              found enum `Option<(fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}, &str)>`

error[E0277]: `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}` doesn't implement `Debug`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
...  |
44 | |                     $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::None);
   | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs:16:5
   |
16 |       assert_eq!(pns("*so* has a %% escape"), Some((S::Escape, " escape")));
   |
   |
   = help: the trait `Debug` is not implemented for `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}`
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:39:35
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected enum `printf::Substitution`, found fn item
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs:17:5
   |
17 |       assert_eq!(pns("%% leading escape"), Some((S::Escape, " leading escape")));
   |
   |
   = note: expected enum `Option<(printf::Substitution<'_>, &str)>`
              found enum `Option<(fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}, &str)>`

error[E0277]: `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}` doesn't implement `Debug`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
...  |
44 | |                     $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::None);
   | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs:17:5
   |
17 |       assert_eq!(pns("%% leading escape"), Some((S::Escape, " leading escape")));
   |
   |
   = help: the trait `Debug` is not implemented for `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}`
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:39:35
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected enum `printf::Substitution`, found fn item
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs:18:5
   |
18 |       assert_eq!(pns("trailing escape %%"), Some((S::Escape, "")));
   |
   |
   = note: expected enum `Option<(printf::Substitution<'_>, &str)>`
              found enum `Option<(fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}, &str)>`

error[E0277]: `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}` doesn't implement `Debug`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
...  |
44 | |                     $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::None);
   | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs:18:5
   |
18 |       assert_eq!(pns("trailing escape %%"), Some((S::Escape, "")));
   |
   |
   = help: the trait `Debug` is not implemented for `fn((usize, usize)) -> printf::Substitution<'_> {printf::Substitution::<'_>::Escape}`
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_builtin_macros` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
