
error[E0507]: cannot move out of borrowed content
  --> src/main.rs:12:39
   |
1  | struct Test(u64, u64);
   | ---------------------- `Test` doesn't implement `Clone`
...
12 |     let _tuples = x.iter().map(|test| test.clone().into_tuple()).collect::<Vec<_>>();
   |                                       ^^^^^^^^^^^^ cannot move out of borrowed content
   |
   = help: `test` is of type `&Test` and `Test` doesn't implement `Clone`, so you're cloning the borrow

error: aborting due to previous error
