
error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
  --> $DIR/issue-59488.rs:6:9
   |
LL |     foo > 12;
   |     --- ^ -- {integer}
   |     |
   |     fn() -> i32 {foo}
   |     did you forget `()`?
   |
   = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> i32 {foo}`
