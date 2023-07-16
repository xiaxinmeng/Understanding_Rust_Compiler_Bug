plain
   Compiling tinyvec v0.3.4
   Compiling byteorder v1.3.4
   Compiling ryu v1.0.5
   Compiling regex-syntax v0.6.22
error[E0596]: cannot borrow data in an index of `Vec<structs::DisplayLine<'_>>` as mutable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:345:33
345 | ...                   ref mut inline_marks,
345 | ...                   ref mut inline_marks,
    |                       ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
    |
    = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `Vec<structs::DisplayLine<'_>>`

error[E0596]: cannot borrow data in an index of `Vec<structs::DisplayLine<'_>>` as mutable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:383:29
383 | ...                   ref mut inline_marks,
383 | ...                   ref mut inline_marks,
    |                       ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
    |
    = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `Vec<structs::DisplayLine<'_>>`

error[E0596]: cannot borrow data in an index of `Vec<structs::DisplayLine<'_>>` as mutable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:398:29
398 | ...                   ref mut inline_marks,
398 | ...                   ref mut inline_marks,
    |                       ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
    |
    = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `Vec<structs::DisplayLine<'_>>`
   Compiling serde_json v1.0.59
   Compiling itoa v0.4.6
   Compiling snap v1.0.1
   Compiling ansi_term v0.12.1
