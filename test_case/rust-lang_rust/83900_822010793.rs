plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:474:31
    |
474 |     ImportSource { did, path, attrs }
    |                               ^^^^^ expected struct `std::boxed::Box`, found struct `types::Attributes`
    |
    = note: expected enum `std::option::Option<std::boxed::Box<types::Attributes>>`
               found enum `std::option::Option<types::Attributes>`
error[E0308]: mismatched types
   --> src/librustdoc/html/render/print_item.rs:291:61
    |
    |
291 |                     let import_item = clean::Item { def_id, attrs, ..myitem.clone() };
    |                                                             ^^^^^ expected struct `types::Attributes`, found struct `std::boxed::Box`
    = note: expected struct `std::boxed::Box<types::Attributes>`
    = note: expected struct `std::boxed::Box<types::Attributes>`
               found struct `std::boxed::Box<std::boxed::Box<types::Attributes>>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
