plain
    Checking unicode-normalization v0.1.13
error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.19/src/fallback.rs:834:31
    |
834 |                 text.extend(c.escape_debug());
    |                               |
    |                               expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/library/core/src/char/methods.rs:466:12
    |
466 |     pub fn escape_debug(self, is_char: bool) -> EscapeDebug {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.19/src/fallback.rs:848:27
    |
    |
848 |             text.extend(t.escape_debug());
    |                           |
    |                           expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/library/core/src/char/methods.rs:466:12
    |
466 |     pub fn escape_debug(self, is_char: bool) -> EscapeDebug {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
