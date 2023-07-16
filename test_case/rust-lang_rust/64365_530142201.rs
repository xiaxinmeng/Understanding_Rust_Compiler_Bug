rust
error[E0596]: cannot borrow `guard` as mutable, as it is not declared as mutable
   --> crates/sled/src/iter.rs:118:21
    |
92  |         let (mut pid, mut node, guard) =
    |                                 ----- help: consider changing this to be mutable: `mut guard`
...
118 |                     guard.repin();
    |                     ^^^^^ cannot borrow as mutable
