
error[E0594]: cannot assign to `*some_name` which is behind a `&` reference
  --> src/main.rs:10:38
   |
10 |         E::Variant(ref some_name) => *some_name += 10,
   |                                      ^^^^^^^^^^^^^^^^ cannot assign
