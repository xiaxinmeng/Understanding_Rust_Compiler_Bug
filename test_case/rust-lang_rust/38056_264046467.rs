
failures:

---- string::String::split_off_0 stdout ----
	error: use of unstable library feature 'string_split_off' (see issue #38080)
 --> <anon>:5:19
  |
5 | let world = hello.split_off(7);
  |                   ^^^^^^^^^
  |
  = help: add #![feature(string_split_off)] to the crate attributes to enable

error: aborting due to previous error(s)

thread 'string::String::split_off_0' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:198
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    string::String::split_off_0
