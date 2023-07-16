
error[E0382]: use of moved value: `r`
 --> src/main.rs:5:14
  |
3 |     let mut r = &mut a;
  |         ----- move occurs because `r` has type `&mut i32`, which does not implement the `Copy` trait
4 |     let v = unsafe { std::ptr::replace(&mut r as *mut _, r) };
  |                                                          - value moved here
5 |     let v2 = r;
  |              ^ value used here after move
