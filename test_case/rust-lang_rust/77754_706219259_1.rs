
error[E0596]: cannot borrow `f` as mutable, as it is a captured variable in a `Fn` closure
   --> compiler/rustc_middle/src/ty/trait_def.rs:127:13
    |
127 |             f(did);
    |             ^ cannot borrow as mutable
