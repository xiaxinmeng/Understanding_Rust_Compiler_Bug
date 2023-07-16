text
   Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
  --> src/main.rs:29:18
   |
29 | type D<'a, 'b> = <M<'a> as Machine>::Datum<'b>;
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected named lifetime parameter
   |
note: these named lifetimes are available to use
  --> src/main.rs:29:8
   |
29 | type D<'a, 'b> = <M<'a> as Machine>::Datum<'b>;
   |        ^^  ^^

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to previous error
