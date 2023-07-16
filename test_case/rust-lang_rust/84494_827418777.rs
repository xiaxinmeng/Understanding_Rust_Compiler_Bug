plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/inline.rs:308:38
    |
308 |                 Attributes::from_ast(both, None)
    |                                      |
    |                                      |
    |                                      expected `&[Attribute]`, found struct `Vec`
    |                                      help: consider borrowing here: `&both`
    |
    = note: expected reference `&[Attribute]`
                  found struct `Vec<Attribute>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
