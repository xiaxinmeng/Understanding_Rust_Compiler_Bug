
warning: cannot borrow `h` as mutable because it is also borrowed as immutable
 --> src/main.rs:7:3
  |
6 |   let a = h.get("a").unwrap();
  |           - immutable borrow occurs here
7 |   h.insert(String::from("b"), format!("{}", a));
  |   ^ mutable borrow occurs here              - immutable borrow later used here
  |
  = note: `#[warn(mutable_borrow_reservation_conflict)]` on by default
  = warning: this borrowing pattern was not meant to be accepted, and may become a hard error in the future
  = note: for more information, see issue #59159 <https://github.com/rust-lang/rust/issues/59159>
