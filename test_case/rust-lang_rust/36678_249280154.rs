
failures:

---- [ui] ui/span/issue-36530.rs stdout ----
    ui: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/tmp/distcheck/rustc-nightly/src/test/ui/span/issue-36530.rs
normalized stderr:
error: The attribute `foo` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
  --> $DIR/issue-36530.rs:11:1
   |
11 | #[foo]
   | ^^^^^^

error: The attribute `foo` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
  --> $DIR/issue-36530.rs:13:5
   |
13 |     #![foo]
   |     ^^^^^^^

error: aborting due to 2 previous errors



expected stderr:
error: The attribute `foo` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
  --> $DIR/issue-36530.rs:11:1
   |
11 | #[foo]
   | ^^^^^^
   |
   = help: add #![feature(custom_attribute)] to the crate attributes to enable

error: The attribute `foo` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
  --> $DIR/issue-36530.rs:13:5
   |
13 |     #![foo]
   |     ^^^^^^^
   |
   = help: add #![feature(custom_attribute)] to the crate attributes to enable

error: aborting due to 2 previous errors
