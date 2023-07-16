
error[E0690]: transparent struct needs exactly one non-zero-sized field, but has 0
 --> src/lib.rs:2:1
  |
2 | struct TransparentCustomZst(());
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ needs exactly one non-zero-sized field, but has 0

error: aborting due to previous error
