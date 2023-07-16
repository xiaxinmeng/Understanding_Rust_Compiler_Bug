
error[E0596]: cannot borrow immutable borrowed content `*t` as mutable
 --> src/main.rs:2:5
  |
1 | fn test(t: &Iterator<Item = &u64>) {
  |            ---------------------- use `&mut Iterator<Item = &u64>` here to make mutable
2 |     t.next();
  |     ^ cannot borrow as mutable
