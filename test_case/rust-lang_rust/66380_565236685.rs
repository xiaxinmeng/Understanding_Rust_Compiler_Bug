
error[E0191]: the value of the associated type `Error` (from trait `Feather`) must be specified
  --> file12.rs:24:87
   |
2  |     type Error;
   |     ----------- `Error` defined here
...
24 | fn pick_feather<'a>(first: bool, f1: &'a dyn Feather, f2: &'a dyn Feather) -> &'a dyn Feather {
   |                                                                                       ^^^^^^^ help: specify the associated type: `Feather<Error = Type>`

error[E0191]: the value of the associated type `Error` (from trait `Feather`) must be specified
  --> file12.rs:24:46
   |
2  |     type Error;
   |     ----------- `Error` defined here
...
24 | fn pick_feather<'a>(first: bool, f1: &'a dyn Feather, f2: &'a dyn Feather) -> &'a dyn Feather {
   |                                              ^^^^^^^ help: specify the associated type: `Feather<Error = Type>`

error[E0191]: the value of the associated type `Error` (from trait `Feather`) must be specified
  --> file12.rs:24:67
   |
2  |     type Error;
   |     ----------- `Error` defined here
...
24 | fn pick_feather<'a>(first: bool, f1: &'a dyn Feather, f2: &'a dyn Feather) -> &'a dyn Feather {
   |                                                                   ^^^^^^^ help: specify the associated type: `Feather<Error = Type>`

error: aborting due to 3 previous errors
