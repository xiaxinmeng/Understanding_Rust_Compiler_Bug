
error[E0597]: `self` does not live long enough
  --> src/main.rs:17:32
   |
17 |         ORDER.iter().map(|id| &self.children[*id as usize])
   |                           --   ^^^^ `self` would have to be valid for `'_`...
   |                           |
   |                           has type `&ChildId`
18 |     }
   |     - ...but `self` will be dropped here, when the function `ordered_children` returns
   |
   = note: functions cannot return a borrow to data owned within the function's scope, functions can only return borrows to data passed as arguments
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references>
