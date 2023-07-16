text
error[E0423]: cannot initialize a tuple struct which contains private fields
  --> src/main.rs:4:13
   |
4  |     let _ = SetCookie(vec![]);
   |             ^^^^^^^^^
   |
note: constructor is not visible here due to private fields
  --> /home/waffle/.cargo/registry/src/github.com-1ecc6299db9ec823/headers-0.2.3/src/common/set_cookie.rs:56:22
   |
56 | pub struct SetCookie(Vec<::HeaderValue>);
   |                      ^^^^^^^^^^^^^^^^^^ private field
