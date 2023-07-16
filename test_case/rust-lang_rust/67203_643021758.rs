
error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
  --> src/main.rs:17:26
   |
17 |         ORDER.iter().map(|id| &self.children[*id as usize])
   |                          ^^^^  ---- `self` is borrowed here
   |                          |
   |                          may outlive borrowed value `self`
   |
note: closure is returned here
  --> src/main.rs:15:35
   |
15 |     fn ordered_children(&self) -> impl Iterator<Item = &Child> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
   |
17 |         ORDER.iter().map(move |id| &self.children[*id as usize])
   |                          ^^^^^^^^^
