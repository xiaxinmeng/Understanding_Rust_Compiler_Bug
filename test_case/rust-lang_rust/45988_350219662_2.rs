
lunch-box. rustc --stage1 -Znll -Zborrowck=mir ~/tmp/issue-45988.rs
error[E0506]: cannot assign to `x` because it is borrowed
 --> /home/nmatsakis/tmp/issue-45988.rs:6:5
  |
5 |     let p = &x;
  |             -- borrow of `x` occurs here
6 |     x += 1; // point A
  |     ^^^^^^ assignment to borrowed `x` occurs here
7 |     read(p); // point B
  |        - borrowed reference later used here
