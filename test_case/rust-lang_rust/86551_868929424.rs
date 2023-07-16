
    Checking helix-term v0.2.0 (/home/ivan/src/pickfire/rs/helix/helix-term)
error[E0432]: unresolved imports `helix_core::graphemes::ensure_grapheme_boundary`, `helix_core::graphemes::ensure_grapheme_boundary_byte`
  --> helix-term/src/ui/editor.rs:11:17
   |
11 |     graphemes::{ensure_grapheme_boundary, ensure_grapheme_boundary_byte},
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `ensure_grapheme_boundary_byte` in `graphemes`
   |                 |
   |                 no `ensure_grapheme_boundary` in `graphemes`
   |                 help: a similar name exists in the module: `prev_grapheme_boundary`

error[E0603]: struct `Rect` is private
 --> helix-term/src/ui/info.rs:3:18
  |
3 | use tui::layout::Rect;
  |                  ^^^^ private struct
  |
note: the struct `Rect` is defined here
 --> /home/ivan/src/pickfire/rs/helix/helix-tui/src/layout.rs:8:36
  |
8 | use helix_view::graphics::{Margin, Rect};
  |                                    ^^^^

error[E0609]: no field `autoinfo` on type `&mut helix_view::Editor`
    --> helix-term/src/commands.rs:3288:15
     |
3288 |     cx.editor.autoinfo = Some(("hello".to_owned(), String::new()));
     |               ^^^^^^^^ unknown field
     |
     = note: available fields are: `tree`, `documents`, `count`, `selected_register`, `registers` ... and 6 others

error[E0609]: no field `autoinfo` on type `&mut helix_view::Editor`
   --> helix-term/src/ui/editor.rs:782:68
    |
782 |         if let Some((title, body)) = std::mem::take(&mut cx.editor.autoinfo) {
    |                                                                    ^^^^^^^^ unknown field
    |
    = note: available fields are: `tree`, `documents`, `count`, `selected_register`, `registers` ... and 6 others

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0603, E0609.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `helix-term`

To learn more, run the command again with --verbose.
