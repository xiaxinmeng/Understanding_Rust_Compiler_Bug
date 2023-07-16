
> cargo check
    Checking helix-term v0.3.0 (/home/ivan/src/pickfire/rs/helix/helix-term)
error[E0425]: cannot find function `move_next_long_word_start` in module `movement`
   --> helix-term/src/commands.rs:449:38
    |
449 |         .transform(|range| movement::move_next_long_word_start(text, range, count));
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: a function with a similar name exists
    |
449 |         .transform(|range| movement::move_next_word_start(text, range, count));
    |                                      ^^^^^^^^^^^^^^^^^^^^
help: consider importing this function
    |
1   | use crate::commands::insert::move_next_long_word_start;
    |

error[E0425]: cannot find function `move_prev_long_word_start` in module `movement`
   --> helix-term/src/commands.rs:461:38
    |
461 |         .transform(|range| movement::move_prev_long_word_start(text, range, count));
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: a function with a similar name exists
    |
461 |         .transform(|range| movement::move_prev_word_start(text, range, count));
    |                                      ^^^^^^^^^^^^^^^^^^^^
help: consider importing this function
    |
1   | use crate::commands::insert::move_prev_long_word_start;
    |

error[E0425]: cannot find function `move_next_long_word_end` in module `movement`
   --> helix-term/src/commands.rs:473:38
    |
473 |         .transform(|range| movement::move_next_long_word_end(text, range, count));
    |                                      ^^^^^^^^^^^^^^^^^^^^^^^
    |
help: a function with a similar name exists
    |
473 |         .transform(|range| movement::move_next_word_end(text, range, count));
    |                                      ^^^^^^^^^^^^^^^^^^
help: consider importing this function
    |
1   | use crate::commands::insert::move_next_long_word_end;
    |

error[E0412]: cannot find type `LspFormatting` in module `helix_lsp::util`
    --> helix-term/src/commands.rs:1934:51
     |
1934 |     format: impl Future<Output = helix_lsp::util::LspFormatting> + Send + 'static,
     |                                                   ^^^^^^^^^^^^^ not found in `helix_lsp::util`

error[E0599]: no method named `document_mut` found for mutable reference `&mut helix_view::Editor` in the current scope
    --> helix-term/src/commands.rs:1939:35
     |
1939 |         if let Some(doc) = editor.document_mut(doc_id) {
     |                                   ^^^^^^^^^^^^ help: there is an associated function with a similar name: `documents_mut`

error[E0599]: no method named `auto_format` found for mutable reference `&mut helix_view::Document` in the current scope
    --> helix-term/src/commands.rs:1165:23
     |
1165 |         let fmt = doc.auto_format().map(|fmt| {
     |                       ^^^^^^^^^^^ method not found in `&mut helix_view::Document`

error[E0599]: no method named `format_and_save` found for mutable reference `&mut helix_view::Document` in the current scope
    --> helix-term/src/commands.rs:1176:29
     |
1176 |         Ok(tokio::spawn(doc.format_and_save(fmt)))
     |                             ^^^^^^^^^^^^^^^ method not found in `&mut helix_view::Document`

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> helix-term/src/commands.rs:1196:35
     |
1196 |         if let Some(format) = doc.format() {
     |                                   ^^^^^^- supplied 0 arguments
     |                                   |
     |                                   expected 1 argument
     |
note: associated function defined here

error[E0308]: mismatched types
    --> helix-term/src/commands.rs:1196:16
     |
1196 |         if let Some(format) = doc.format() {
     |                ^^^^^^^^^^^^   ------------ this expression has type `()`
     |                |
     |                expected `()`, found enum `std::option::Option`
     |
     = note: expected unit type `()`
                     found enum `std::option::Option<_>`

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0061, E0308, E0412, E0425, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `helix-term`

To learn more, run the command again with --verbose.
