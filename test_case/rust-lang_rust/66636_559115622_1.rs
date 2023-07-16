rust
note: ...the reference/variable is valid for the lifetime 'a as defined on the impl at 27:1...
  --> src\lib.rs:27:1
   |
27 | / impl<'a> MyImpl<'a>{
28 | |
29 | |     pub fn myfunc(i: &String) -> MyImpl{
30 | |         ...
...  |
89 | |     }
90 | | }
   | |_^
