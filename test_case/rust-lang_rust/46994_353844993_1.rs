
error[E0597]: borrowed value does not live long enough
 --> ...\test.rs:4:12
  |
4 |     a[0] = 0.to_string().as_bytes()[0];
  |            ^^^^^^^^^^^^^               - temporary value only lives until here
  |            |
  |            temporary value does not live long enough
  |
  = note: borrowed value must be valid for lifetime '_#3r...
