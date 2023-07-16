plain
    Checking addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared type `NewImpl`
   --> library/std/src/ffi/c_str.rs:411:17
    |
411 |                 NewImpl::new_impl(self.as_bytes())
    |                 ^^^^^^^ use of undeclared type `NewImpl`
error[E0433]: failed to resolve: use of undeclared type `NewImpl`
   --> library/std/src/ffi/c_str.rs:417:17
    |
    |
417 |                 NewImpl::new_impl(&self[..])
    |                 ^^^^^^^ use of undeclared type `NewImpl`

error[E0405]: cannot find trait `NewImpl` in this scope
    |
    |
409 |         impl NewImpl for &'_ str {


error[E0405]: cannot find trait `NewImpl` in this scope
    |
    |
415 |         impl NewImpl for &'_ mut [u8] {

Some errors have detailed explanations: E0405, E0433.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `std` due to 4 previous errors
