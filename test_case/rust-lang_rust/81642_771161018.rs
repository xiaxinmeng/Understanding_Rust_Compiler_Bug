plain
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: trailing semicolon in macro used in expression position
   --> compiler/rustc_builtin_macros/src/deriving/into.rs:22:16
    |
13  | / macro_rules! invalid_derive {
14  | |     ($cx:ident, $span:ident, $reason:expr) => {
15  | |         struct_span_err!(
16  | |             &$cx.sess.parse_sess.span_diagnostic,
...   |
22  | |         .emit();
23  | |     };
24  | | }
24  | | }
    | |_- in this expansion of `invalid_derive!`
...
141 |               _ => invalid_derive!(cx, span, "enums"),
    |
    |
    = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
    = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

error: trailing semicolon in macro used in expression position
   --> compiler/rustc_builtin_macros/src/deriving/into.rs:22:16
   --> compiler/rustc_builtin_macros/src/deriving/into.rs:22:16
    |
13  | / macro_rules! invalid_derive {
14  | |     ($cx:ident, $span:ident, $reason:expr) => {
15  | |         struct_span_err!(
16  | |             &$cx.sess.parse_sess.span_diagnostic,
...   |
22  | |         .emit();
23  | |     };
24  | | }
24  | | }
    | |_- in this expansion of `invalid_derive!`
...
143 |           _ => invalid_derive!(cx, span, "enums"),
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

