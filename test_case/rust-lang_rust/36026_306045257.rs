
error[E0597]: borrowed value does not live long enough
 --> test.rs:2:42
  |
2 |     let stderr = std::io::stderr().lock();
  |                  -----------------       ^ temporary value dropped here while still borrowed
  |                  |
  |                  temporary value created here
3 | }
  | - temporary value needs to live until here
  |
  = note: consider using a `let` binding to increase its lifetime

error: aborting due to previous error(s)
