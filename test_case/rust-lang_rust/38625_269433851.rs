
error[E0282]: unable to infer enough type information about `_`
    --> src/libcollections/vec_deque.rs:2497:17
     |
2497 |             let expected = (0..).take(len).collect();
     |                 ^^^^^^^^ cannot infer type for `_`
     |
     = note: type annotations or generic parameter binding required

error: aborting due to previous error
