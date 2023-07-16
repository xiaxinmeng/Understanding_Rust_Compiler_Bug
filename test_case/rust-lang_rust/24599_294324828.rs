
warning: function is never used: `test`
 --> <anon>:2:1
  |
2 |   fn test() -> (u8,) {
  |  _^ starting here...
3 | |     (-1.0 as u8,)
4 | | }
  | |_^ ...ending here
  |
  = note: #[warn(dead_code)] on by default

warning: can't cast this type
 --> <anon>:3:6
  |
3 |     (-1.0 as u8,)
  |      ^^^^^^^^^^
  |
  = note: #[warn(const_err)] on by default

warning: can't cast this type
 --> <anon>:3:6
  |
3 |     (-1.0 as u8,)
  |      ^^^^^^^^^^
  |
  = note: #[warn(const_err)] on by default

warning: can't cast this type
 --> <anon>:3:6
  |
3 |     (-1.0 as u8,)
  |      ^^^^^^^^^^
  |
  = note: #[warn(const_err)] on by default
