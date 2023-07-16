
error[E0507]: cannot move out of borrowed content
  --> src/main.rs:11:15
   |
11 |         match *self {
   |               ^^^^^ cannot move out of borrowed content
12 |             Animal::Cat(c) => f.write_str("c"),
   |                         - hint: to prevent move, use `ref c` or `ref mut c`
