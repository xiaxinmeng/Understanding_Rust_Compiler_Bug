
error[E0596]: cannot borrow immutable argument `v` as mutable
 --> src/main.rs:4:20
  |
3 | fn f(v: Vec<RefCell<u8>>) {
  |      - consider changing this to `mut v`
4 |     let _t = &mut *v[0].borrow_mut();
  |                    ^ cannot borrow mutably
