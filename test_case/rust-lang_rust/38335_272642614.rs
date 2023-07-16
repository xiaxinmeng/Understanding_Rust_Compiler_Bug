
warning: function cannot return without recurring, #[warn(unconditional_recursion)] on by default
 --> <anon>:3:1
  |
3 |   fn rec<F>(f: F) -> i64
  |  _^ starting here...
4 | |     where F: Fn(i64) -> i64 {
5 | |     rec(|x|{f(x)})
6 | | }
  | |_^ ...ending here
  |
note: recursive call site
 --> <anon>:5:5
  |
5 |     rec(|x|{f(x)})
  |     ^^^^^^^^^^^^^^
  = help: a `loop` may express intention better if this is on purpose

error: reached the type-length limit while instantiating `rec::<[closure@<anon>:5:9: 5:18 f:&[closure@<anon>:5:9: 5:18 f:&...`
 --> <anon>:3:1
  |
3 |   fn rec<F>(f: F) -> i64
  |  _^ starting here...
4 | |     where F: Fn(i64) -> i64 {
5 | |     rec(|x|{f(x)})
6 | | }
  | |_^ ...ending here
  |
  = note: consider adding a `#![type_length_limit="2097152"]` attribute to your crate

error: aborting due to previous error
