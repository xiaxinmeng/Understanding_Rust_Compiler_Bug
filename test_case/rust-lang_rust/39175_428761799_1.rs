
   Compiling bad_ffi_ext_trait_suggestion v0.1.0 (file:///C:/Users/egubler/AppData/Local/Cargo/script-cache/file-bad_ffi_ext_trait_suggestion-eaa1f7e82798a3de)
error[E0599]: no function or associated item named `from_wide` found for type `std::ffi::OsString` in the current scope
 --> bad_ffi_ext_trait_suggestion.rs:4:13
  |
4 |     let _ = OsString::from_wide(b"a\x00b\x00c\x00");
  |             ^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::ffi::OsString`
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
1 | use std::os::ext::ffi::OsStringExt;
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: Could not compile `bad_ffi_ext_trait_suggestion`.

To learn more, run the command again with --verbose.
