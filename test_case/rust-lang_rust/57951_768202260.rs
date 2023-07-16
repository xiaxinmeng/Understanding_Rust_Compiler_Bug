
error[E0423]: expected function, tuple struct or tuple variant, found struct `SetCookie`
  --> src/main.rs:4:17
   |
4  |         let _ = SetCookie(vec![]);
   |                 ^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `SetCookie { 0: val }`
   | 
  ::: /home/vext01/.cargo/registry/src/github.com-1ecc6299db9ec823/headers-0.2.3/src/common/set_cookie.rs:56:1
   |
56 | pub struct SetCookie(Vec<::HeaderValue>);
   | ----------------------------------------- `SetCookie` defined here
