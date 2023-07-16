
warning: value assigned to `ve` is never read
   --> build/cg/request.rs:710:33
    |
710 |                         let mut ve = String::new();
    |                                 ^^
    |
    = note: `#[warn(unused_assignments)]` on by default
    = help: maybe it is overwritten before being read?
    = help: consider to declare the binding without assigning a value.
    |
710 |                         let ve;
    |                         ^^^^^^^
    |
