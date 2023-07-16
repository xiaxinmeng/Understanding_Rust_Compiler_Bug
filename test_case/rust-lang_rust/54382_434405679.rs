
error[E0597]: `_thing1` does not live long enough
 --> src/main.rs:6:25
  |
4 |        {
  |   _____-
  |  |_____|
  | ||
5 | ||         let mut _thing1 = D(Box::new("thing1"));
6 | ||         D("other").next(&_thing1)
  | ||                         ^^^^^^^^ borrowed value does not live long enough
7 | ||     }
  | ||     -
  | ||     |
  | ||_____`_thing1` dropped here while still borrowed
  | |______a temporary with access to the borrow is created here ...
  |        ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `D`
