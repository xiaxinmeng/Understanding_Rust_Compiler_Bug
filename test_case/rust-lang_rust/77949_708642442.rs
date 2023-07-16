
error[E0271]: type mismatch resolving `for<'a> <[closure@<source>:28:18: 28:23] as std::ops::FnOnce<(&'a i32,)>>::Output == _`
  --> <source>:28:5
   |
28 |     call_handler(|x| x);
   |     ^^^^^^^^^^^^ expected bound lifetime parameter 'a, found concrete lifetime
   |
   = note: required because of the requirements on the impl of `for<'a> Handler<'a>` for `[closure@<source>:28:18: 28:23]`
note: required by `call_handler`
  --> <source>:17:1
   |
17 | fn call_handler(handler: impl for<'a> Handler<'a>) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0631]: type mismatch in closure arguments
  --> <source>:28:5
   |
28 |     call_handler(|x| x);
   |     ^^^^^^^^^^^^ ----- found signature of `fn(_) -> _`
   |     |
   |     expected signature of `for<'a> fn(&'a i32) -> _`
   |
   = note: required because of the requirements on the impl of `for<'a> Handler<'a>` for `[closure@<source>:28:18: 28:23]`
note: required by `call_handler`
  --> <source>:17:1
   |
17 | fn call_handler(handler: impl for<'a> Handler<'a>) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^