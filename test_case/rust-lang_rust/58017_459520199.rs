
error[E0450]: cannot invoke tuple struct constructor with private fields
 --> <source>:6:13
  |
2 |     pub struct Error(usize);
  |                      ------ private field declared here
...
6 |     let x = a::Error(3);
  |             ^^^^^^^^ cannot construct with a private field
