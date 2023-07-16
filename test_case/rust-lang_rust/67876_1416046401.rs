
error[E0382]: use of moved value: `v`
 --> src/main.rs:3:14
  |
1 | fn foo(v: &mut Vec<i32>) {
  |        - move occurs because `v` has type `&mut Vec<i32>`, which does not implement the `Copy` trait
2 |     for x in v {}
  |              - `v` moved due to this implicit call to `.into_iter()`
3 |     for x in v {}
  |              ^ value used here after move
  |
note: this function takes ownership of the receiver `self`, which moves `v`
 --> /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/iter/traits/collect.rs:262:18
help: consider creating a fresh reborrow of `v` here
  |
2 |     for x in &mut *v {}
  |              ++++++
