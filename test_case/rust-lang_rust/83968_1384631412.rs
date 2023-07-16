
error[E0425]: cannot find function `memcpy` in this scope
 --> src/main.rs:5:5
  |
5 |     memcpy(&dst, &src[2..]);
  |     ^^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
1 | use libc::memcpy;
  |
1 | use openssl_sys::memcpy;
  |

error[E0599]: no method named `memcpy` found for array `[{integer}; 2]` in the current scope
 --> src/main.rs:6:9
  |
6 |     dst.memcpy(&src[2..]);
  |         ^^^^^^ method not found in `[{integer}; 2]`
