
warning: `<` is interpreted as a start of generic arguments for `usize`, not comparison
  --> $DIR/issue-22644.rs:16:33
   |
16 |     println!("{}", a as usize < b);
   |                               - ^ interpreted as generic argument
   |                               |
   |                               not interpreted as comparison
   |
help: if you want to compare the casted value then write:
   |     println!("{}", (a as usize) < b);
