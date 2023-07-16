bash
(base) C02XF1TZJHD3:rust pkoura$ grep -rh --include "*.stderr" "/issues/" src/test/ui | sed -E 's/\#[0-9]+/#<ISSUE>/g' | sed -E 's/issues\/[0-9]+/issues\/<ISSUE>/g' | sort -u
           see https://github.com/rust-lang/rust/issues/<ISSUE> for more details
   = note: for more information, see https://github.com/rust-lang/rust/issues/<ISSUE>
   = note: for more information, see issue #<ISSUE> <https://github.com/rust-lang/rust/issues/<ISSUE>>
   = note: for more information, see issue https://github.com/rust-lang/rust/issues/<ISSUE>
   = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/<ISSUE> for more info). Consider whether you meant to use the type `()` here instead.
error: use of deprecated attribute `no_debug`: the `#[no_debug]` attribute was an experimental feature that has been deprecated due to lack of demand. See https://github.com/rust-lang/rust/issues/<ISSUE>
note: use of `&&` operator here does not actually short circuit due to the const evaluator presently not being able to do control flow. See https://github.com/rust-lang/rust/issues/<ISSUE> for more information.
warning: use of deprecated attribute `no_debug`: the `#[no_debug]` attribute was an experimental feature that has been deprecated due to lack of demand. See https://github.com/rust-lang/rust/issues/<ISSUE>
