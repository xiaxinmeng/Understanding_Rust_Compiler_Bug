
error[E0381]: use of possibly-uninitialized variable: `val`
 --> src/main.rs:6:14
  |
6 |         dbg!(val, bind_me);
  |              ^^^ use of possibly-uninitialized `val`

error[E0381]: use of possibly-uninitialized variable: `bind_me`
 --> src/main.rs:6:19
  |
6 |         dbg!(val, bind_me);
  |                   ^^^^^^^ use of possibly-uninitialized `bind_me`
