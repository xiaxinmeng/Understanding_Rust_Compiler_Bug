rust
#![feature(no_core, staged_api, rustc_attrs, lang_items, decl_macro)]
#![no_core]

#![stable(feature = "rust", since ="1.0")]

#[rustc_builtin_macro]
#[stable(feature = "matches_macro", since = "1.42.0")]
pub macro matches($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) {{ /* compiler built-in */ }}

#[lang = "sized"]
trait Sized {}

enum Option<T> {
    Some(T),
    None
}

use Option::Some;

fn main() {
    matches!(Some(1), Some(x) if x == 2);
}
