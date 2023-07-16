
error[E0061]: this function takes 6 arguments but 2 arguments were supplied
  --> src/main.rs:2:5
   |
2  |     g((), ());
   |     ^ --  -- supplied 2 arguments
   |     |
   |     expected 6 arguments
   |
note: function defined here
  --> src/main.rs:5:8
   |
5  |   pub fn g(
   |          ^
6  |       a1: (),
   |       ------
7  |       a2: bool,
   |       --------
8  |       a3: bool,
   |       --------
9  |       a4: bool,
   |       --------
10 |       a5: bool,
   |       --------
11 | /     a6: ()
12 | | ) -> () {
   | |_
