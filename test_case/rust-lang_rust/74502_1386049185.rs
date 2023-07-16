
error: lifetime may not live long enough
  --> lib.rs:4:5
   |
3  |   fn with_build_args<'a, 'b, 'c, 'd>(app: Box<App<'a, 'b>>) -> Box<App<'c, 'd>> {
   |                      --      -- lifetime `'c` defined here
   |                      |
   |                      lifetime `'a` defined here
4  | /     Box::new(
5  | |         (*app)
6  | |             .arg(
7  | |                 Arg::with_name("release")
...  |
10 | |             )
11 | |     )
   | |_____^ function was supposed to return data with lifetime `'c` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'c`

error: lifetime may not live long enough
  --> lib.rs:4:5
   |
3  |   fn with_build_args<'a, 'b, 'c, 'd>(app: Box<App<'a, 'b>>) -> Box<App<'c, 'd>> {
   |                          --      -- lifetime `'d` defined here
   |                          |
   |                          lifetime `'b` defined here
4  | /     Box::new(
5  | |         (*app)
6  | |             .arg(
7  | |                 Arg::with_name("release")
...  |
10 | |             )
11 | |     )
   | |_____^ function was supposed to return data with lifetime `'d` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'd`

help: the following changes may resolve your lifetime errors
  |
  = help: add bound `'a: 'c`
  = help: add bound `'b: 'd`
