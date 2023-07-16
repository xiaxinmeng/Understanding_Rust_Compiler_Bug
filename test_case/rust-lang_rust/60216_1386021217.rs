
error[E0106]: missing lifetime specifiers
 --> lib.rs:3:32
  |
3 | pub fn cli_args() -> clap::App<'_, '_> {
  |                                ^^  ^^ expected named lifetime parameter
  |                                |
  |                                expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
3 | pub fn cli_args() -> clap::App<'static, 'static> {
  |                                ~~~~~~~  ~~~~~~~
