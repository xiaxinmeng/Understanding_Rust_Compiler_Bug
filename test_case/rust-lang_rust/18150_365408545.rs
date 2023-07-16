
error[E0594]: cannot assign to field of immutable binding
 --> src/main.rs:4:5
  |
4 |     x.a += 1;
  |     ^^^^^^^^ cannot mutably borrow field of immutable binding

error[E0594]: cannot assign to field `y.a` of immutable binding
 --> src/main.rs:7:5
  |
7 |     y.a += 1;
  |     ^^^^^^^^ cannot mutably borrow field of immutable binding
