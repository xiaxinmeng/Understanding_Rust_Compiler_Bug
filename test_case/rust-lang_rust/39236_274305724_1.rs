
rustc 1.16.0-nightly (f0b420759 2017-01-19)
warning: function is never used: `foo`, #[warn(dead_code)] on by default
 --> <anon>:1:1
  |
1 |   fn foo() {  
  |  _^ starting here...
2 | |     if true {}  
3 | | }
  | |_^ ...ending here
