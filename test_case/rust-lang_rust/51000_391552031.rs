
error[E0XXX]: cannot add-assign `x` to itself because `Int` doesn't implement `Copy`
  --> $DIR/augmented-assignments.rs:26:5
   |
LL |     x 
   |     ^ cannot add-assign here...
LL |     +=
   |     -- add-assign borrows the assign target as mutable # This span is not available now
LL |     x;
   |     - ...because `x` is borrowed here
   = note: you cannot add-assign using the same binding on both side of the operation unless the type implements `Copy`
