text
error: borrowed value does not live long enough
  --> foo.rs:9:11
   |
9  |     g2(&1);
   |         - ^ temporary value dropped here while still borrowed
   |         |
   |         temporary value created here
10 | }
   | - temporary value needs to live until here
   |
   = note: consider using a `let` binding to increase its lifetime
