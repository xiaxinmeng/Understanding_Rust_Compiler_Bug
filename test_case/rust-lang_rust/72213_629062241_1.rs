text
✗ cargo check
    Checking foo v0.1.0 (/tmp/tmp.NxnOdGgulp/foo)
warning: struct is never constructed: `Lines`
 --> src/lib.rs:1:1
  |
1 | / struct Lines<'a, L>
2 | | where
3 | |     L: Iterator<Item = &'a ()>,
4 | | {
5 | |     words: std::iter::Peekable<Words<'a, L>>,
6 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: struct is never constructed: `Words`
 --> src/lib.rs:8:1
  |
8 | struct Words<'a, L> {
  | ^^^^^^^^^^^^^^^^^^^
