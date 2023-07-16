
error[E0597]: `bt` does not live long enough
  --> src/lib.rs:19:5
   |
17 | async fn actual<'a, BT>(bt: BT) where BT: BoolTrait<'a> {
   |                 -- lifetime `'a` defined here
18 |     let v = 42;
19 |     bt.check(&v).await;
   |     ^^^^^^^^^^^^
   |     |
   |     borrowed value does not live long enough
   |     argument requires that `bt` is borrowed for `'a`
20 | }
   | - `bt` dropped here while still borrowed

error[E0309]: the parameter type `BT` may not live long enough
  --> src/lib.rs:19:5
   |
19 |     bt.check(&v).await;
   |     ^^^^^^^^^^^^ ...so that the type `BT` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound...
   |
17 | async fn actual<'a, BT>(bt: BT) where BT: BoolTrait<'a> + 'a {
   |                                                         ++++
