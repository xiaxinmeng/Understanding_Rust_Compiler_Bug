
3 |   let pos: &[&str] = pos.iter().map(AsRef::as_ref).collect::<Vec<_>>().as_slice();
  |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^           - temporary value is freed at the end of this statement
  |                      |
  |                      creates a temporary which is freed while still in use
4 |   println!("{:?}", pos);
  |                    --- borrow later used here
  |
help: consider using a `let` binding to create a longer lived value
  |
3 ~   let binding = pos.iter().map(AsRef::as_ref).collect::<Vec<_>>();
4 ~   let pos: &[&str] = binding.as_slice();
