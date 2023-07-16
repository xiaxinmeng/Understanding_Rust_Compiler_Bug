
lunch-box. rustc --stage1 ~/tmp/issue-43058.rs  -Znll -Zborrowck=mir
error[E0597]: borrowed value does not live long enough
  --> /home/nmatsakis/tmp/issue-43058.rs:24:2
   |
16 |     let s2 = [S { name: Cow::Borrowed("Test3") }, S { name: Cow::Borrowed("Test4") }];
   |         -- temporary value created here
...
24 | }
   |  ^ temporary value dropped here while still borrowed
   |
   = note: consider using a `let` binding to increase its lifetime

error: aborting due to previous error
