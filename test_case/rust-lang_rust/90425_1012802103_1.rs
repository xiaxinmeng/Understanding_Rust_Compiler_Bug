rust
error[E0499]: cannot borrow `v` as mutable more than once at a time
 --> src/main.rs:4:17
  |
3 |       v.push({
  |       - ---- first borrow later used by call
  |  _____|
  | |
4 | |         let x = v.pop().unwrap();
  | |                 ^^^^^^^ second mutable borrow occurs here
5 | |         x
6 | |     });
  | |______- first mutable borrow occurs here
  |
help: try adding a local storing this argument...
 --> src/main.rs:4:17
  |
4 |         let x = v.pop().unwrap();
  |                 ^^^^^^^
help: ...and then using that local as the argument to this call
 --> src/main.rs:3:5
  |
3 | /     v.push({
4 | |         let x = v.pop().unwrap();
5 | |         x
6 | |     });
  | |______^
