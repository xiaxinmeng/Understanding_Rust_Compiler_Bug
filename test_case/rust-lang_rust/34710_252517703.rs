
error: borrowed value does not live long enough
 --> <anon>:4:13
  |
4 |     let x = Command::new("curl").arg("foo");
  |             ^^^^^^^^^^^^^^^^^^^^           - temporary value only lives until here
  |             |
  |             temporary value created here
5 | }
  | - temporary value needs to live until here
  |
  = note: consider using a `let` binding to increase its lifetime
