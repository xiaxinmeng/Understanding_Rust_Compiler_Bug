
error[E0308]: mismatched types
  --> src/main.rs:18:11
   |
18 | fn f() -> impl Send {
   |           ^^^^^^^^^ one type is more general than the other
   |
   = note: expected reference `&()`
              found reference `&()`
