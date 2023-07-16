
error[E0382]: use of moved value: `a`
  --> src/main.rs:17:11
   |
14 |         [_, _, s2_5 @ .., _, _] => { println!("sub2_5 {:?}", s2_5); }
   |                --------- value moved here
...
17 |     match a {
   |           ^ value used here after partial move
   |
   = note: move occurs because `a[..]` has type `D`, which does not implement the `Copy` trait

error: aborting due to previous error
