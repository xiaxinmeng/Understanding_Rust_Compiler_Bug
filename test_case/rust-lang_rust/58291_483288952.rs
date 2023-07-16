
% cargo clean && echo ---- THE DIFF ---- && diff -u src/impls_v1.rs src/impls_v2.rs ; echo ---- IMPL 1 ---- && cp src/impls_v1.rs src/impls.rs && cargo +nightly check && \
cp src/impls_v2.rs src/impls.rs && echo ---- IMPL 2 ---- && cargo +nightly check
---- THE DIFF ----
--- src/impls_v1.rs     2019-04-15 14:33:59.693052503 +0200
+++ src/impls_v2.rs     2019-04-15 14:34:03.190103348 +0200
@@ -2,5 +2,6 @@
 use crate::SourceDatabase;
 use crate::RootDatabase;

+
 pub(crate) fn goto_implementation(db: &RootDatabase) -> u32
 { db.parse(); loop { } }
---- IMPL 1 ----
    Checking ra_ide_api v0.1.0 (/tmp/issue_58291/ra_ide_api)
warning: function is never used: `goto_implementation`
 --> src/impls.rs:5:1
  |
5 | pub(crate) fn goto_implementation(db: &RootDatabase) -> u32
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
---- IMPL 2 ----
    Checking ra_ide_api v0.1.0 (/tmp/issue_58291/ra_ide_api)
error[E0599]: no method named `parse` found for type `&RootDatabase` in the current scope
 --> src/impls.rs:7:6
  |
7 | { db.parse(); loop { } }
  |      ^^^^^
  |
  = note: the method `parse` exists but the following trait bounds were not satisfied:
          `&RootDatabase : SourceDatabase`
          `RootDatabase : SourceDatabase`
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `parse`, perhaps you need to implement it:
          candidate #1: `SourceDatabase`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: Could not compile `ra_ide_api`.

To learn more, run the command again with --verbose.
%
