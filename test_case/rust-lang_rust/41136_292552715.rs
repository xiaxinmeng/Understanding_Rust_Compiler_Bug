rust
error[E0046]: not all trait items implemented, missing: `Item`
  --> $DIR/issue-23729.rs:20:9
   |
20 |           impl Iterator for Recurrence {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `Item` in implementation
   |
   = note: `Item` from trait: `type Item;`
