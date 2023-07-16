
error[E0271]: type mismatch resolving `for<'r, 's> <[closure@src/main.rs:7:35: 7:56] as std::ops::FnOnce<(&'r regex::Captures<'s>,)>>::Output == std::string::String`
 --> src/main.rs:7:20
  |
7 |     let after = re.replace_all(s, |c| "xxx".to_string());
  |                    ^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
  |
  = note: required because of the requirements on the impl of `regex::Replacer` for `[closure@src/main.rs:7:35: 7:56]`

error[E0631]: type mismatch in closure arguments
 --> src/main.rs:7:20
  |
7 |     let after = re.replace_all(s, |c| "xxx".to_string());
  |                    ^^^^^^^^^^^    --------------------- found signature of `fn(_) -> _`
  |                    |
  |                    expected signature of `for<'r, 's> fn(&'r regex::Captures<'s>) -> _`
  |
  = note: required because of the requirements on the impl of `regex::Replacer` for `[closure@src/main.rs:7:35: 7:56]`

error: aborting due to 2 previous errors
