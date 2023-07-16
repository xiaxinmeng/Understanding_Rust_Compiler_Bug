plain
---- src/collections/linked_list.rs - collections::linked_list::LinkedList<T>::insert (line 442) stdout ----
error[E0308]: mismatched types
  --> src/collections/linked_list.rs:449:1
   |
10 | assert_eq!(list, [1, 4, 2, 3]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `LinkedList`, found array `[{integer}; 4]`
   = note: expected struct `LinkedList<{integer}>`
   = note: expected struct `LinkedList<{integer}>`
               found array `[{integer}; 4]`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
