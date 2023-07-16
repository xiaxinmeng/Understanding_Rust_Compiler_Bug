
error[E0106]: missing lifetime specifier
 --> main.rs:1:62
  |
1 | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
  |                     ------------------------------------     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of `iter`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
  |
1 | fn parse_type<'a>(iter: Box<dyn Iterator<Item=&'a str>+'static>) -> &'a str { iter.next() }
  |              ++++                              ++                    ++

error[E0308]: mismatched types
 --> main.rs:1:69
  |
1 | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
  |                                                              ----   ^^^^^^^^^^^ expected `&'static str`, found enum `Option`
  |                                                              |
  |                                                              expected `&'static str` because of return type
  |
  = note: expected reference `&'static str`
                  found enum `Option<&str>`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0106, E0308.
For more information about an error, try `rustc --explain E0106`.
