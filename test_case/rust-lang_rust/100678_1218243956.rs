plain
   Compiling jsondocck v0.1.0 (/checkout/src/tools/jsondocck)
error[E0308]: mismatched types
  --> src/tools/jsondocck/src/cache.rs:19:57
   |
19 |         let file_path = root.join(&Path::with_extension(filename, "json"));
   |                                    -------------------- ^^^^^^^^ expected struct `std::path::Path`, found struct `OsStr`
   |                                    arguments to this function are incorrect
   |
   = note: expected reference `&std::path::Path`
              found reference `&OsStr`
              found reference `&OsStr`
note: associated function defined here

For more information about this error, try `rustc --explain E0308`.
error: could not compile `jsondocck` due to previous error
