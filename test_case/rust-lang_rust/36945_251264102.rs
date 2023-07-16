
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
   --> src/libserialize/json.rs:436:26
    |
436 |     escape_str(writer, v.encode_utf8(&mut [0; 4]))
    |                          ^^^^^^^^^^^ expected 0 parameters

error[E0308]: mismatched types
   --> src/libserialize/json.rs:436:24
    |
436 |     escape_str(writer, v.encode_utf8(&mut [0; 4]))
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected &str, found struct `rustc_unicode::char::EncodeUtf8`
    |
    = note: expected type `&str`
    = note:    found type `rustc_unicode::char::EncodeUtf8`

error: aborting due to 2 previous errors
