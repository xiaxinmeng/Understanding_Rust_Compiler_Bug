plain
---- src/option.rs - option::Option<&'amutOption<T>>::flatten_ref (line 2647) stdout ----
error[E0716]: temporary value dropped while borrowed
 --> src/option.rs:2650:45
  |
6 | let x: Option<&mut Option<u32>> = Some(&mut Some(6));
  |                                             ^^^^^^^ - temporary value is freed at the end of this statement
  |                                             creates a temporary value which is freed while still in use
  |                                             creates a temporary value which is freed while still in use
7 | assert_eq!(&Some(6), x.flatten_ref());
  |
help: consider using a `let` binding to create a longer lived value
  |
6 + let binding = Some(6);
6 + let binding = Some(6);
7 ~ let x: Option<&mut Option<u32>> = Some(&mut binding);

error[E0716]: temporary value dropped while borrowed
  --> src/option.rs:2653:45
   |
   |
9  | let x: Option<&mut Option<u32>> = Some(&mut None);
   |                                             ^^^^ - temporary value is freed at the end of this statement
   |                                             creates a temporary value which is freed while still in use
   |                                             creates a temporary value which is freed while still in use
10 | assert_eq!(&None, x.flatten_ref());
   |
help: consider using a `let` binding to create a longer lived value
   |
9  + let binding = None;
9  + let binding = None;
10 ~ let x: Option<&mut Option<u32>> = Some(&mut binding);

error: aborting due to 2 previous errors


error: doctest failed, to rerun pass `-p core --doc`
Couldn't compile the test.

failures:
    src/option.rs - option::Option<&'amutOption<T>>::flatten_ref (line 2647)
