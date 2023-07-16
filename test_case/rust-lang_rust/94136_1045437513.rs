plain
........................................................iiiiii...................................... 1200/1232
i...............................
failures:

---- src/fs.rs - fs::append (line 319) stdout ----
error[E0658]: use of unstable library feature 'fs_append_bytes'
  |
  |
8 |     fs::append("foo.txt", b"Lorem ipsum")?;
  |
  = note: see issue #94135 <https://github.com/rust-lang/rust/issues/94135> for more information
  = note: see issue #94135 <https://github.com/rust-lang/rust/issues/94135> for more information
  = help: add `#![feature(fs_append_bytes)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'fs_append_bytes'
  |
  |
9 |     fs::append("bar.txt", "dolor sit")?;
  |
  = note: see issue #94135 <https://github.com/rust-lang/rust/issues/94135> for more information
  = note: see issue #94135 <https://github.com/rust-lang/rust/issues/94135> for more information
  = help: add `#![feature(fs_append_bytes)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
