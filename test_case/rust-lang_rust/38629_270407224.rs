
---- char::char::escape_debug_0 stdout ----
	error: use of unstable library feature 'char_escape_debug' (see issue #35068)
 --> <anon>:4:15
  |
4 | for i in '\n'.escape_debug() {
  |               ^^^^^^^^^^^^
  |
  = help: add #![feature(char_escape_debug)] to the crate attributes to enable

error: aborting due to previous error(s)

thread 'char::char::escape_debug_0' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- char::char::escape_debug_1 stdout ----
	error: use of unstable library feature 'char_escape_debug' (see issue #35068)
 --> <anon>:4:26
  |
4 | let quote: String = '\n'.escape_debug().collect();
  |                          ^^^^^^^^^^^^
  |
  = help: add #![feature(char_escape_debug)] to the crate attributes to enable

error: aborting due to previous error(s)

thread 'char::char::escape_debug_1' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
