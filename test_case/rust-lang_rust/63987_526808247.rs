
warning: multiple patterns covering the same range
  --> $DIR/issue-43253.rs:42:9
   |
LL |         5..7 => {},
   |         ---- this range overlaps on `5i32..=6i32`
LL |         6 => {},
   |         - this range overlaps on `6i32`
LL |         1..10 => {},
   |         ^^^^^ overlapping patterns
