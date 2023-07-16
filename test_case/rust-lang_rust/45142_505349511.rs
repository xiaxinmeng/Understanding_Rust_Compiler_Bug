
error[E0382]: use of moved value: `value`
  --> src/main.rs:22:37
   |
22 |                     Other::try_from(value);
   |                                     ^^^^^ value moved here in previous iteration of loop
   |
   = note: move occurs because `value` has type `&'a mut xml::EventReader<R>`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `*value`
  --> src/main.rs:27:20
   |
22 |                     Other::try_from(value);
   |                                     ----- value moved here
...
27 |             next = value.next();
   |                    ^^^^^ value used here after move
   |
   = note: move occurs because `value` has type `&'a mut xml::EventReader<R>`, which does not implement the `Copy` trait

error: aborting due to 2 previous errors
