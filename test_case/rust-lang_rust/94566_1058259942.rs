plain
   Compiling test v0.0.0 (/checkout/library/test)
error[E0308]: mismatched types
   --> library/test/src/formatters/pretty.rs:223:36
    |
223 |                 self.write_ignored(desc.ignore_message)?;
    |                                    ^^^^^^^^^^^^^^^^^^^ expected struct `String`, found `&str`
    = note: expected enum `Option<String>`
               found enum `Option<&'static str>`

For more information about this error, try `rustc --explain E0308`.
