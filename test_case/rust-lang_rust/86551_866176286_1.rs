
    Checking helix-lsp v0.2.0 (/home/ivan/src/pickfire/rs/helix/helix-lsp)
    Checking helix-tui v0.2.0 (/home/ivan/src/pickfire/rs/helix/helix-tui)
error[E0432]: unresolved import `helix_core::chars::char_is_line_ending`
 --> helix-lsp/src/client.rs:6:18
  |
6 | use helix_core::{chars::char_is_line_ending, find_root, ChangeSet, Rope};
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `char_is_line_ending` in `chars`

error[E0432]: unresolved import `helix_core::line_ending`
  --> helix-tui/src/text.rs:50:17
   |
50 | use helix_core::line_ending::str_is_line_ending;
   |                 ^^^^^^^^^^^ could not find `line_ending` in `helix_core`

error[E0432]: unresolved import `helix_core::line_ending`
 --> helix-tui/src/widgets/reflow.rs:2:17
  |
2 | use helix_core::line_ending::str_is_line_ending;
  |                 ^^^^^^^^^^^ could not find `line_ending` in `helix_core`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `helix-lsp`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
error: build failed
