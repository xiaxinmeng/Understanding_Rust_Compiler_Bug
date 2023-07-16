
error[E0597]: `s` does not live long enough
  --> src/main.rs:10:22
   |
10 |     if let Some(o) = s.borrow().get("a") {
   |                      ^---------
   |                      |
   |                      borrowed value does not live long enough
   |                      a temporary with access to the borrow is created here ...
...
16 | }
   | -
   | |
   | `s` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, std::collections::HashMap<std::string::String, std::string::String>>`
   |
   = note: The temporary is part of an expression at the end of a block. Consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped.
