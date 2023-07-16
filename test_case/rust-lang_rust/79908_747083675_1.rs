
error: expected expression, found keyword `const`
  --> src/main.rs:99:19
   |
47 |     (exp $e:expr) => {
   |          ------- while parsing argument for this `expr` macro fragment
...
99 |         stmt!(exp const 1);
   |                   ^^^^^ expected expression

error: expected expression, found keyword `const`
   --> src/main.rs:100:19
    |
47  |     (exp $e:expr) => {
    |          ------- while parsing argument for this `expr` macro fragment
...
100 |         stmt!(exp const 2);
    |                   ^^^^^ expected expression

error: expected expression, found keyword `const`
   --> src/main.rs:101:19
    |
47  |     (exp $e:expr) => {
    |          ------- while parsing argument for this `expr` macro fragment
...
101 |         stmt!(exp const 3);
    |                   ^^^^^ expected expression
