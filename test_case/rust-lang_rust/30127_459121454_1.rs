
error[E0597]: borrowed value does not live long enough
 --> src/main.rs:5:21
  |
5 |     let _desugar1 = "a".to_owned().index(..);
  |                     ^^^^^^^^^^^^^^          - temporary value dropped here while still borrowed
  |                     |
  |                     temporary value does not live long enough
6 |     let _desugar2 = &*"a".to_owned().index(..);
7 | }
  | - temporary value needs to live until here
  |
  = note: consider using a `let` binding to increase its lifetime

error[E0597]: borrowed value does not live long enough
 --> src/main.rs:6:23
  |
6 |     let _desugar2 = &*"a".to_owned().index(..);
  |                       ^^^^^^^^^^^^^^          - temporary value dropped here while still borrowed
  |                       |
  |                       temporary value does not live long enough
7 | }
  | - temporary value needs to live until here
  |
  = note: consider using a `let` binding to increase its lifetime

error: aborting due to 2 previous errors
