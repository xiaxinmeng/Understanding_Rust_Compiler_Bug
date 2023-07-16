
error[E0597]: borrowed value does not live long enough
 --> src/main.rs:2:19
  |
2 |   let x = Some(&0);
  |                 - ^ temporary value dropped here while still borrowed
  |                 |
  |                 temporary value created here
3 | }
  | - temporary value needs to live until here
  |
  = note: consider using a `let` binding to increase its lifetime
