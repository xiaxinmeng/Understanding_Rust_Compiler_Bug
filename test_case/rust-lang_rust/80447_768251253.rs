
error: expected one of `)` or `,`, found `{`
 --> src/lib.rs:3:18
  |
3 |     fn one(s: () {
  |           -     -^
  |           |     |
  |           |     help: `)` may belong here
  |           unclosed delimiter

error: non-item in item list
 --> src/lib.rs:5:1
  |
1 | pub trait Crash {
  |                 - item list starts here
...
5 | }
  | ^
  | |
  | non-item starts here
  | item list ends here

error: expected one of `!` or `::`, found `:`
 --> src/lib.rs:2:5
  |
2 |     #[proc_macro::mac]
  |     ^^^^^^^^^^^^^^^^^^

error: aborting due to 3 previous errors

error: could not compile `issue`

To learn more, run the command again with --verbose.
