
error[E0277]: the trait bound `&char: Issue<&mut _>` is not satisfied
  --> src/main.rs:35:17
   |
35 |     let _ = ARR.make_container().do_something();
   |                 ^^^^^^^^^^^^^^ the trait `Issue<&mut _>` is not implemented for `&char`
   |
   = help: the following implementations were found:
             <&'static char as Issue<()>>
