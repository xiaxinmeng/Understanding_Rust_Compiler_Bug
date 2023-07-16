
error: this file contains an un-closed delimiter
   --> src/future.rs:105:3
    |
105 | }
    |   ^
    |
help: did you mean to close this delimiter?
   --> src/future.rs:20:19
    |
20  | impl<T> Future<T> {
    |                   ^

error: expected one of `::` or `:`, found `,`
  --> src/future.rs:66:31
   |
66 |     pub fn thenVal<F, U>(&self, func : F) -> U
   |                               ^ expected one of `::` or `:` here

error: expected one of `::` or `:`, found `)`
  --> src/future.rs:75:23
   |
75 |     pub fn value(&self) -> Result<T, io::Error> {
   |                       ^ expected one of `::` or `:` here

error: expected one of `const`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found `mod`
  --> src/future.rs:85:1
   |
84 | #[cfg(test)]
   |             - expected one of 7 possible tokens here
85 | mod tests {
   | ^^^ unexpected token

warning: expected `;`, found `<eof>`
  --> src/scopeguard.rs:35:47
   |
35 |         let gensym = ScopeGuard::new(|| { $e })
   |                                               ^
   |
   = note: This was erroneously allowed and will become a hard error in a future release

warning: expected `;`, found `<eof>`
  --> src/scopeguard.rs:35:47
   |
35 |         let gensym = ScopeGuard::new(|| { $e })
   |                                               ^
   |
   = note: This was erroneously allowed and will become a hard error in a future release

error[E0401]: can't use type parameters from outer function; try using a local type parameter instead
  --> src/future.rs:67:29
   |
67 |         where F: FnOnce(Try<T>) -> U
   |                             ^ use of type variable from outer function

error[E0401]: can't use type parameters from outer function; try using a local type parameter instead
  --> src/future.rs:75:35
   |
75 |     pub fn value(&self) -> Result<T, io::Error> {
   |                                   ^ use of type variable from outer function

error[E0434]: can't capture dynamic environment in a fn item; use the || { ... } closure form instead
  --> src/future.rs:76:9
   |
76 |         self.panic_if_invalid();
   |         ^^^^

error[E0434]: can't capture dynamic environment in a fn item; use the || { ... } closure form instead
  --> src/future.rs:78:22
   |
78 |             return (*self.core_ptr).get_try().value();
   |                      ^^^^

error: method `set_interrupt_handler_nolock` is private
  --> src/future.rs:62:27
   |
62 |             (*p.core_ptr).set_interrupt_handler_nolock(self.core.get_interrupt_handler());
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: no field `core` on type `&future::Future<T>`
  --> src/future.rs:62:61
   |
62 |             (*p.core_ptr).set_interrupt_handler_nolock(self.core.get_interrupt_handler());
   |                                                             ^^^^

error: aborting due to 2 previous errors
