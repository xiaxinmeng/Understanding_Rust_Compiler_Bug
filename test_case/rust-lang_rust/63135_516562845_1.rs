
   Compiling lolr v0.1.0 (/home/lampam/cpp/throwaway/lolr)
error: this file contains an un-closed delimiter
 --> src/main.rs:1:16
  |
1 | fn i(n{...,f #
  |     - -        ^
  |     | |
  |     | un-closed delimiter
  |     un-closed delimiter

error: expected field pattern, found `...`
 --> src/main.rs:1:8
  |
1 | fn i(n{...,f #
  |        ^^^ help: to omit remaining fields, use one fewer `.`: `..`

error: expected `}`, found `,`
 --> src/main.rs:1:11
  |
1 | fn i(n{...,f #
  |        ---^
  |        |  |
  |        |  expected `}`
  |        `..` must be at the end and cannot have a trailing comma

error: expected `[`, found `}`
 --> src/main.rs:1:15
  |
1 | fn i(n{...,f #
  |               ^ expected `[`

error: expected `:`, found `)`
 --> src/main.rs:1:15
  |
1 | fn i(n{...,f #
  |               ^ expected `:`

error: expected one of `->`, `where`, or `{`, found `<eof>`
 --> src/main.rs:1:15
  |
1 | fn i(n{...,f #
  |               ^ expected one of `->`, `where`, or `{` here

error: aborting due to 6 previous errors

error: Could not compile `lolr`.

To learn more, run the command again with --verbose.
