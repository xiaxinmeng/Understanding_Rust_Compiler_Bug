
error[E0504]: cannot move `f` into closure because it is borrowed
  --> src/main.rs:21:20
   |
21 |     f(map(|a| hylo(f, g, a), g(x)))
   |     -              ^ move into closure occurs here
   |     |
   |     borrow of `f` occurs here

error[E0382]: use of moved value: `g`
  --> src/main.rs:21:30
   |
21 |     f(map(|a| hylo(f, g, a), g(x)))
   |           ---                ^ value used here after move
   |           |
   |           value moved (into closure) here
   |
   = note: move occurs because `g` has type `F1`, which does not implement the `Copy` trait

error[E0507]: cannot move out of captured outer variable in an `Fn` closure
  --> src/main.rs:21:20
   |
16 | fn hylo<F0, F1, S, T0, T1>(f: F0, g: F1, x: T1) -> T0
   |                            - captured outer variable
...
21 |     f(map(|a| hylo(f, g, a), g(x)))
   |                    ^ cannot move out of captured outer variable in an `Fn` closure

error[E0507]: cannot move out of captured outer variable in an `Fn` closure
  --> src/main.rs:21:23
   |
16 | fn hylo<F0, F1, S, T0, T1>(f: F0, g: F1, x: T1) -> T0
   |                                   - captured outer variable
...
21 |     f(map(|a| hylo(f, g, a), g(x)))
   |                       ^ cannot move out of captured outer variable in an `Fn` closure

error: aborting due to 4 previous errors

Some errors occurred: E0382, E0504, E0507.
For more information about an error, try `rustc --explain E0382`.
