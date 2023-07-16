
% rustc file.rs -Zborrowck=compare
error[E0596]: cannot borrow immutable item `self` as mutable (Mir)
 --> file.rs:7:21
  |
7 |             let r = &mut self;
  |                     ^^^^^^^^^ cannot borrow as mutable
  |
  = note: Value not mutable causing this error: `self`

error[E0595]: closure cannot assign to immutable argument `self` (Ast)
 --> file.rs:6:9
  |
5 |     fn foo(&mut self) {
  |            --------- given this existing mutable borrow...
6 |         || {
  |         ^^ cannot reborrow immutable argument `self` mutably in this closure

error[E0596]: cannot borrow immutable item `self` as mutable (Mir)
 --> file.rs:6:9
  |
6 | /         || {
7 | |             let r = &mut self;
8 | |             r.u += 1;
9 | |         };
  | |_________^ cannot borrow as mutable

error: aborting due to 3 previous errors
