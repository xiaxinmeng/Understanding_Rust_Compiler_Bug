
$ ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc ~/Desktop/bug.rs 
error: this file contains an unclosed delimiter
 --> /home/dwrensha/Desktop/bug.rs:1:33
  |
1 | struct B{C:[);{#![cfg_attr(r(c:
  |         -     -  -        - -   ^
  |         |     |  |        | |
  |         |     |  |        | unclosed delimiter
  |         |     |  |        unclosed delimiter
  |         |     |  unclosed delimiter
  |         |     unclosed delimiter
  |         unclosed delimiter

error: expected type, found `]`
 --> /home/dwrensha/Desktop/bug.rs:1:13
  |
1 | struct B{C:[);{#![cfg_attr(r(c:
  |             ^ expected type

error: expected one of `.`, `?`, `]`, or an operator, found `}`
 --> /home/dwrensha/Desktop/bug.rs:1:33
  |
1 | struct B{C:[);{#![cfg_attr(r(c:
  |            - unclosed delimiter ^ help: `]` may belong here

error[E0658]: attributes on expressions are experimental
 --> /home/dwrensha/Desktop/bug.rs:1:16
  |
1 | struct B{C:[);{#![cfg_attr(r(c:
  |                ^^^^^^^^^^^^^^^^^
  |
  = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
  = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

error: expected one of `(`, `)`, `,`, `::`, or `=`, found `:`
 --> /home/dwrensha/Desktop/bug.rs:1:31
  |
1 | struct B{C:[);{#![cfg_attr(r(c:
  |                               ^ expected one of `(`, `)`, `,`, `::`, or `=`

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
