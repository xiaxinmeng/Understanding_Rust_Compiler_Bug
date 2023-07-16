
error[E0308]: mismatched types
  --> src/tools/rustbook/src/main.rs:63:40
   |
63 |                             .unwrap_or(false)
   |                                        ^^^^^ expected struct `mdbook_linkcheck::errors::BrokenLinks`, found bool
   |
   = note: expected type `mdbook_linkcheck::errors::BrokenLinks`
              found type `bool`

error[E0599]: no method named `contains` found for type `&std::boxed::Box<dyn mdbook_linkcheck::errors::BrokenLink>` in the current scope
  --> src/tools/rustbook/src/main.rs:67:49
   |
67 |                             .any(|cause| !cause.contains("timed out"));
   |                                                 ^^^^^^^^

error[E0308]: mismatched types
  --> src/tools/rustbook/src/main.rs:76:20
   |
76 |                 if actually_broken {
   |                    ^^^^^^^^^^^^^^^
   |                    |
   |                    expected bool, found ()
   |                    in this expansion of `desugaring of `if` or `while` condition`
   |                    in this macro invocation
   |
   = note: expected type `bool`
              found type `()`
