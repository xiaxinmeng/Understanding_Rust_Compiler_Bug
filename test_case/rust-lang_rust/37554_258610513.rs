
error: `r` does not live long enough
  --> <anon>:21:9
   |
18 |     r.listen(Box::new(|| {
   |                       -- capture occurs here
...
21 |         r.something();
   |         ^ does not live long enough
22 |     }));
23 | }
   | - borrowed value dropped before borrower
