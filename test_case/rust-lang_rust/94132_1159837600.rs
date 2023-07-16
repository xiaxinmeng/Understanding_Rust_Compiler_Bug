
   Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:6:20
  |
6 |     T: Serialize + Deserialize,
  |                    ^^^^^^^^^^^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
4 ~ pub enum Example<'a, T>
5 | where
6 ~     T: Serialize + Deserialize<'a>,
  |

error[E0106]: missing lifetime specifier
 --> src/lib.rs:6:20
  |
6 |     T: Serialize + Deserialize,
  |                    ^^^^^^^^^^^ expected named lifetime parameter
  |
help: consider using the `'de` lifetime
  |
6 |     T: Serialize + Deserialize<'de>,
  |                    ~~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to 2 previous errors
