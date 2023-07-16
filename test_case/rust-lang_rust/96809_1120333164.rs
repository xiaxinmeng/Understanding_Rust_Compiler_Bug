
error[E0599]: no method named `unwrap` found for fn item `fn(Vec<u8>) -> Result<String, FromUtf8Error> {String::from_utf8}` in the current scope
  --> src/lib.rs:83:71
   |
83 |     let spec_name = (-uppercase_char * ident).map((String::from_utf8).unwrap());
   |                                                   ------------------- ^^^^^^ method not found in `fn(Vec<u8>) -> Result<String, FromUtf8Error> {String::from_utf8}`
   |                                                   |
   |                                                   this is a function, perhaps you wish to call it

For more information about this error, try `rustc --explain E0599`.
error: could not compile `configurator` due to previous error
