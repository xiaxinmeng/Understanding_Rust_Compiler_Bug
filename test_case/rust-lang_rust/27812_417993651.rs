
warning: a method with this name may be added to the standard library in the future
  --> src\parser\lexer.rs:38:31
   |
38 |       Some((start, ch)) if ch.is_xid_start() => {
   |                               ^^^^^^^^^^^^
   |
   = note: #[warn(unstable_name_collisions)] on by default
   = warning: once this method is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `unicode_xid::UnicodeXID::is_xid_start(...)` to keep using the current method
   = note: add #![feature(rustc_private)] to the crate attributes to enable `std::char::methods::<impl char>::is_xid_start`
