
lunch-box. rustc --stage1 ~/tmp/arielb3.rs -Znll -Zborrowck=mir -Znll-dump-cause -Zdump-mir=nll
error[E0506]: cannot assign to `b` because it is borrowed
  --> /home/nmatsakis/tmp/arielb3.rs:43:9
   |
32 |     let c_b = Cell::new(&b);
   |                         -- borrow of `b` occurs here
...
43 |         b = 1;
   |         ^^^^^ assignment to borrowed `b` occurs here
   |
   = note: because of a outlives relation created at `bb2[6]`
           because of a outlives relation created at `bb2[6]`
           because of a outlives relation created at `bb3[0]`
           because of a outlives relation created at `bb3[9]`
           because of a outlives relation created at `bb3[10]`
           because of a outlives relation created at `bb3[10]`
           because of a outlives relation created at `bb4[0]`
           because `_12` is dropped at bb6[0]
