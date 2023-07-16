
error: lifetime may not live long enough
 --> it.rs:7:9
  |
6 |       pub fn wrapping<'a, 'b: 'a>(node: &'a mut Linked<'b>) -> Linked<'a> {
  |                       --  -- lifetime `'b` defined here
  |                       |
  |                       lifetime `'a` defined here
7 | /         Linked {
8 | |             node: Some(&mut node),
9 | |         }
  | |_________^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
  |
  = help: consider adding the following bound: `'a: 'b`
  = note: requirement occurs because of the type `Linked<'_>`, which makes the generic argument `'_` invariant
  = note: the struct `Linked<'a>` is invariant over the parameter `'a`
  = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0596]: cannot borrow `node` as mutable, as it is not declared as mutable
 --> it.rs:8:24
  |
8 |             node: Some(&mut node),
  |                        ^^^^^^^^^ cannot borrow as mutable
  |
note: the binding is already a mutable borrow
 --> it.rs:6:39
  |
6 |     pub fn wrapping<'a, 'b: 'a>(node: &'a mut Linked<'b>) -> Linked<'a> {
  |                                       ^^^^^^^^^^^^^^^^^^
help: try removing `&mut` here
  |
8 -             node: Some(&mut node),
8 +             node: Some(node),
  |

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
