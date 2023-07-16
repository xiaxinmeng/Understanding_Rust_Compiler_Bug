
error: borrowed value does not live long enough
  --> <anon>:11:17
   |
11 |     for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
   |                 ---------------- temporary value created here
...
18 |     }
   |     ^ temporary value dropped before borrower
   |
   = note: values in a scope are dropped in the opposite order they are created
