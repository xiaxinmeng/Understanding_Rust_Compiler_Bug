plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0405]: cannot find trait `ToOwned` in this scope
    --> library/core/src/option.rs:1905:18
     |
1905 | impl<T: ToOwned> ToOwned for Option<T> {

error[E0405]: cannot find trait `ToOwned` in this scope
    --> library/core/src/option.rs:1905:9
     |
     |
1905 | impl<T: ToOwned> ToOwned for Option<T> {

error[E0425]: cannot find value `source` in this scope
    --> library/core/src/option.rs:1920:22
     |
     |
1920 |         match (self, source) {
     |                      ^^^^^^ not found in this scope

error[E0546]: missing 'feature'
    --> library/core/src/option.rs:1904:1
     |
1904 | #[stable(since = "1.61.0")]


error[E0599]: no variant named `Owned` found for enum `Option<T>`
     |
518  | pub enum Option<T> {
518  | pub enum Option<T> {
     | ------------------ variant `Owned` not found here
...
1909 |     fn to_owned(&self) -> Self::Owned {
     |                                 ^^^^^ variant not found in `Option<T>`

error[E0599]: no variant named `Owned` found for enum `Option<T>`
     |
518  | pub enum Option<T> {
518  | pub enum Option<T> {
     | ------------------ variant `Owned` not found here
...
1919 |     fn clone_into(&self, target: &mut Self::Owned) {
     |                                             ^^^^^ variant not found in `Option<T>`
Some errors have detailed explanations: E0405, E0425, E0546, E0599.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `core` due to 6 previous errors
Build completed unsuccessfully in 0:00:04
