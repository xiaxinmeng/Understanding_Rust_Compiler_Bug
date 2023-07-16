text
C:\Users\steve\tmp\foo [master +4 ~0 -0 !]> cargo +stable build
   Compiling vs v0.0.1 (file:///C:/Users/steve/tmp/foo)
error: const fn is unstable (see issue #24111)
 --> src\lib.rs:1:1
  |
1 | / pub const fn foo() {
2 | | }
  | |_^

error: aborting due to previous error

error: Could not compile `vs`.

To learn more, run the command again with --verbose.
C:\Users\steve\tmp\foo [master +4 ~0 -0 !]> cargo +nightly build
   Compiling vs v0.0.1 (file:///C:/Users/steve/tmp/foo)
error: const fn is unstable (see issue #24111)
 --> src\lib.rs:1:1
  |
1 | / pub const fn foo() {
2 | | }
  | |_^
  |
  = help: add #![feature(const_fn)] to the crate attributes to enable

error: aborting due to previous error

error: Could not compile `vs`.

To learn more, run the command again with --verbose.
