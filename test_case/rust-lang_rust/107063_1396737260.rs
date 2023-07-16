
   Compiling libc v0.2.139
   Compiling socket2 v0.4.7
error[E0425]: cannot find function `socketpair` in module `sys`
   --> /home/fpoli/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.7/src/socket.rs:172:14
    |
172 |         sys::socketpair(domain.0, ty.0, protocol)
    |              ^^^^^^^^^^ not found in `sys`
    |
help: consider importing this function
    |
9   | use libc::socketpair;
    |
help: if you import `socketpair`, refer to it directly
    |
172 -         sys::socketpair(domain.0, ty.0, protocol)
172 +         socketpair(domain.0, ty.0, protocol)
    |

error[E0425]: cannot find function `keepalive_time` in module `sys`
    --> /home/fpoli/.cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.7/src/socket.rs:1663:14
     |
1663 |         sys::keepalive_time(self.as_raw())
     |              ^^^^^^^^^^^^^^ help: a constant with a similar name exists: `KEEPALIVE_TIME`
     |
    ::: /home/fpoli/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.139/src/unix/linux_like/mod.rs:964:1
     |
964  | pub const TCP_KEEPIDLE: ::c_int = 4;
     | ------------------------------- similarly named constant `KEEPALIVE_TIME` defined here

For more information about this error, try `rustc --explain E0425`.
error: could not compile `socket2` due to 2 previous errors
