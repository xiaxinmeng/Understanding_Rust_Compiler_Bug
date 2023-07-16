plain
    Checking clippy_lints v0.1.53 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/cargo_common_metadata.rs:102:68
    |
102 |                 if is_empty_str(&package.license) && is_empty_path(&package.license_file) {
    |                                                                    ^^^^^^^^^^^^^^^^^^^^^ expected struct `PathBuf`, found struct `Utf8PathBuf`
    |
    = note: expected reference `&std::option::Option<PathBuf>`
               found reference `&std::option::Option<Utf8PathBuf>`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/cargo_common_metadata.rs:110:34
    |
    |
110 |                 if is_empty_path(&package.readme) {
    |                                  ^^^^^^^^^^^^^^^ expected struct `PathBuf`, found struct `Utf8PathBuf`
    |
    = note: expected reference `&std::option::Option<PathBuf>`
               found reference `&std::option::Option<Utf8PathBuf>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints`
