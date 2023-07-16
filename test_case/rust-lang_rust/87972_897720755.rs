
error[E0106]: missing lifetime specifier
 --> f1.rs:2:11
  |
2 |     const A: &u8;
  |              ^ expected named lifetime parameter
  |
help: consider using the `'static` lifetime
  |
2 |     const A: &'static u8;
  |               +++++++
help: consider introducing a named lifetime parameter
  |
1 ~ trait T<'a> {
2 ~     const A: &'a u8;
  |

error[E0106]: missing lifetime specifier
 --> f1.rs:4:20
  |
4 | trait K { const A: &u8; }
  |                    ^ expected named lifetime parameter
  |
help: consider using the `'static` lifetime
  |
4 | trait K { const A: &'static u8; }
  |                     +++++++
help: consider introducing a named lifetime parameter
  |
4 | trait K<'a> { const A: &'a u8; }
  |        ++++            ~~~
