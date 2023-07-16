rust
rustc 1.17.0-nightly (cab4bff3d 2017-03-21)
error: borrowed value does not live long enough
  --> <anon>:21:26
   |
21 |     let r = map.get(&'a');
   |                      --- ^ temporary value dropped here while still borrowed
   |                      |
   |                      temporary value created here
22 |     assert_eq!(r, Some(&string));
23 | }
   | - temporary value needs to live until here
   |
   = note: consider using a `let` binding to increase its lifetime

error: aborting due to previous error
