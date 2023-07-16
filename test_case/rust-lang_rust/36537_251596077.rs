
error: `x` does not live long enough
  --> src/test/compile-fail/regions-escape-loop-via-variable.rs:21:14
   |
21 |         p = &x; //~ ERROR `x` does not live long enough
   |              ^ does not live long enough
22 |     }
   |     - borrowed value only lives until here
23 | }
   | - borrowed value needs to live until here
