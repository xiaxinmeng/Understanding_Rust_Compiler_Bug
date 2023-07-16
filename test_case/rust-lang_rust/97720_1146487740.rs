plain
   Compiling unicode-width v0.1.8
   Compiling unic-char-range v0.9.0
   Compiling scoped-tls v1.0.0
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:474:9
474 | /         snippet::Snippet {
475 | |             title,
476 | |             footer,
477 | |             slices,
477 | |             slices,
478 | |             opt,
479 | |         }: snippet::Snippet<'a>,
    | |_________|
    |           lifetime mismatch
    |
    |
    = note: expected struct `Snippet<'a>`
               found struct `Snippet<'_>`
note: the anonymous lifetime as defined here...
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:474:18
474 |         snippet::Snippet {
    |                  ^^^^^^^
    |                  ^^^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:472:6
    |
472 | impl<'a> From<snippet::Snippet<'a>> for DisplayList<'a> {

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:474:9
474 | /         snippet::Snippet {
475 | |             title,
476 | |             footer,
477 | |             slices,
477 | |             slices,
478 | |             opt,
479 | |         }: snippet::Snippet<'a>,
    | |_________|
    |           lifetime mismatch
    |
    |
    = note: expected struct `Snippet<'a>`
               found struct `Snippet<'_>`
note: the lifetime `'a` as defined here...
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:472:6
    |
472 | impl<'a> From<snippet::Snippet<'a>> for DisplayList<'a> {
note: ...does not necessarily outlive the anonymous lifetime as defined here
note: ...does not necessarily outlive the anonymous lifetime as defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:474:18
474 |         snippet::Snippet {
    |                  ^^^^^^^

error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:482:35
    |
482 |         if let Some(annotation) = title {
    |
note: first, the lifetime cannot outlive the anonymous lifetime as defined here...
note: first, the lifetime cannot outlive the anonymous lifetime as defined here...
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:474:18
474 |         snippet::Snippet {
    |                  ^^^^^^^
note: ...so that the expression is assignable
note: ...so that the expression is assignable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:482:35
    |
482 |         if let Some(annotation) = title {
    = note: expected `Option<snippet::Annotation<'_>>`
    = note: expected `Option<snippet::Annotation<'_>>`
               found `Option<snippet::Annotation<'_>>`
note: but, the lifetime must be valid for the lifetime `'a` as defined here...
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:472:6
    |
472 | impl<'a> From<snippet::Snippet<'a>> for DisplayList<'a> {
note: ...so that the expression is assignable
note: ...so that the expression is assignable
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/annotate-snippets-0.8.0/src/display_list/from_snippet.rs:500:13
500 |             body,
    |             ^^^^
    |             ^^^^
    = note: expected `Vec<structs::DisplayLine<'a>>`
               found `Vec<structs::DisplayLine<'_>>`
   Compiling unic-common v0.9.0
Some errors have detailed explanations: E0308, E0495.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `annotate-snippets` due to 3 previous errors
