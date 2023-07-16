
error[E0423]: expected function, found macro `env::vav_os`
 --> src/main.rs:4:5
  |
4 |     env::vav_os("PATH");
  |     ^^^^^^^^^^^
help: a function with a similar name exists
  |
4 |     env::var_os("PATH");
  |          ^^^^^^
help: use `!` to invoke the macro
  |
4 |     env::vav_os!("PATH");
  |     ^^^^^^^^^^^^
