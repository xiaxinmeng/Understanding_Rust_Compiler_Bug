
error[E0597]: borrowed value does not live long enough
  --> src/main.rs:7:10
   |
7  |           &match (&test,) {
   |  __________^
8  | |             (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
9  | |         },
   | |_________^ temporary value does not live long enough
...
20 |       );
   |        - temporary value dropped here while still borrowed
21 |   }
   |   - temporary value needs to live until here
   |
   = note: consider using a `let` binding to increase its lifetime
