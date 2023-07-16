
error[E0597]: `s` does not live long enough
  --> src/main.rs:20:13
   |
20 |     let r = s.r();
   |             ^ borrowed value does not live long enough
21 | }
   | -
   | |
   | `s` dropped here while still borrowed
   | borrow might be used here, when `s` is dropped and runs the destructor for type `S<'_>`
