plain
---- src/collections/linked_list.rs - collections::linked_list::LinkedList<T>::insert (line 445) stdout ----
error[E0308]: mismatched types
 --> src/collections/linked_list.rs:451:1
  |
9 | assert_eq!(i.next(), Some(1));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&{integer}`, found integer
  = note: expected enum `Option<&{integer}>`
             found enum `Option<{integer}>`
             found enum `Option<{integer}>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> src/collections/linked_list.rs:452:1
   |
   |
10 | assert_eq!(i.next(), Some(4));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&{integer}`, found integer
   = note: expected enum `Option<&{integer}>`
              found enum `Option<{integer}>`
              found enum `Option<{integer}>`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> src/collections/linked_list.rs:453:1
   |
   |
11 | assert_eq!(i.next(), Some(2));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&{integer}`, found integer
   = note: expected enum `Option<&{integer}>`
              found enum `Option<{integer}>`
              found enum `Option<{integer}>`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> src/collections/linked_list.rs:454:1
   |
   |
12 | assert_eq!(i.next(), Some(3));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&{integer}`, found integer
   = note: expected enum `Option<&{integer}>`
              found enum `Option<{integer}>`
              found enum `Option<{integer}>`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
