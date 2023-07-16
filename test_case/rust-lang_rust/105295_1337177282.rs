
error: Undefined Behavior: constructing invalid value: encountered a dangling reference (use-after-free)
  --> src/lib.rs:21:38
   |
21 |         self.listeners().listen(|m|  r.send(f(m)));
   |                                      ^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (use-after-free)
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside closure at src/lib.rs:21:38
   = note: inside `<std::boxed::Box<dyn std::ops::FnMut(())> as std::ops::FnMut<((),)>>::call_mut` at /Users/cognite/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:2016:9
note: inside `MessageListeners::<'_>::send` at src/lib.rs:16:13
  --> src/lib.rs:16:13
   |
16 |             (*listener)(message.clone());
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `test_message_listeners_map0` at src/lib.rs:43:5
  --> src/lib.rs:43:5
   |
43 |     ml.send(());
   |     ^^^^^^^^^^^
note: inside closure at src/lib.rs:38:34
  --> src/lib.rs:38:34
   |
37 | #[test]
   | ------- in this procedural macro expansion
38 | fn test_message_listeners_map0() {
   |                                  ^
   = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
