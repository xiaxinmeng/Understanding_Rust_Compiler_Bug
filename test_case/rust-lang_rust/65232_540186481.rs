
error[E0308]: mismatched types
  --> /home/nmatsakis/tmp/issue-57936.rs:19:5
   |
19 |     indirect::<fn(&())>(); // OK
   |     ^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `X`
              found type `X`
