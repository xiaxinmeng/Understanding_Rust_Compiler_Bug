
> cargo doc
 Documenting helix-view v0.3.0 (/home/ivan/src/pickfire/rs/helix/helix-view)
    Checking helix-term v0.3.0 (/home/ivan/src/pickfire/rs/helix/helix-term)
error[E0432]: unresolved import `helix_lsp::util::LspFormatting`
  --> helix-view/src/document.rs:19:5
   |
19 | use helix_lsp::util::LspFormatting;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `LspFormatting` in `util`

error: Compilation failed, aborting rustdoc

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
error: could not document `helix-view`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-type lib --crate-name helix_view helix-view/src/lib.rs -o /home/ivan/src/pickfire/rs/helix/target/doc --cfg 'feature="crossterm"' --cfg 'feature="default"' --cfg 'feature="term"' --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/ivan/src/pickfire/rs/helix/target/debug/deps --extern anyhow=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libanyhow-a0acf4362debc8da.rmeta --extern bitflags=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libbitflags-2ac7a49e86426b65.rmeta --extern chardetng=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libchardetng-532041c0d3bdc748.rmeta --extern crossterm=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libcrossterm-4fe54d60ca035608.rmeta --extern encoding_rs=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libencoding_rs-36108c5368738efb.rmeta --extern futures_util=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libfutures_util-fb1ecad0003a1568.rmeta --extern helix_core=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libhelix_core-1889099c790a2b7e.rmeta --extern helix_lsp=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libhelix_lsp-c5e990b435ffbb39.rmeta --extern log=/home/ivan/src/pickfire/rs/helix/target/debug/deps/liblog-084cb22155b94a5e.rmeta --extern once_cell=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libonce_cell-e59f925b50d5e08f.rmeta --extern serde=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libserde-2ca165c59121e480.rmeta --extern slotmap=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libslotmap-4b2710a370e66460.rmeta --extern tokio=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libtokio-112fa73ba8304679.rmeta --extern toml=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libtoml-440c89cfa73486c9.rmeta --extern url=/home/ivan/src/pickfire/rs/helix/target/debug/deps/liburl-22fe0e50f79e623a.rmeta --extern which=/home/ivan/src/pickfire/rs/helix/target/debug/deps/libwhich-fbcf7f9d3d2756ae.rmeta --crate-version 0.3.0` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
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
error: build failed
