rust
error[E0596]: cannot borrow `*current_node` as mutable, as it is behind a `&` reference
  --> src/main.rs:47:17
   |
36 |         let mut current_node: &Option<Box<Node<T>>> = &mut self.root;
   |             ---------------- help: consider changing this to be a mutable reference: `&mut Option<Box<Node<T>>>`
...
47 |                 current_node.as_mut().unwrap().left_child = Some(Box::new(new_node));
   |                 ^^^^^^^^^^^^ `current_node` is a `&` reference, so the data it refers to cannot be borrowed as mutable
