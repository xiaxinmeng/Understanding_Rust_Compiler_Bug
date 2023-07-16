bash
error: expected one of `)`, `,`, or `::`, found `(`
 --> src/main.rs:6:19
  |
6 |     #[derive(parse(from_os_str))]
  |                   ^ expected one of `)`, `,`, or `::`

error[E0432]: unresolved import `structopt`
 --> src/main.rs:1:5
  |
1 | use structopt::StructOpt;
  |     ^^^^^^^^^ use of undeclared type or module `structopt`

error: cannot determine resolution for the derive macro `StructOpt`
 --> src/main.rs:3:10
  |
3 | #[derive(StructOpt)]
  |          ^^^^^^^^^
  |
  = note: import resolution is stuck, try simplifying macro imports

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not compile `bisectit`.

To learn more, run the command again with --verbose.
