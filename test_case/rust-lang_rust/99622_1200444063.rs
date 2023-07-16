
error[E0308]: mismatched types
  --> src/main.rs:10:13
   |
10 |             b.iter().fold(true, |acc, x| acc && (*x < 1000))
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `bool`
   |
help: consider using a semicolon at the end of the expression
   |
10 |             b.iter().fold(true, |acc, x| acc && (*x < 1000));
   |                                                             +
help: consider using a semicolon here
   |
10 |             b.iter().fold(true, |acc, x| acc && (*x < 1000));
   |                                                             +
