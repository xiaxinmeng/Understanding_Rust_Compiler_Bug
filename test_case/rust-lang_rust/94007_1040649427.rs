
error[E0107]: this struct takes 3 generic arguments but 2 generic arguments were supplied
   --> /rustc/1.58.1/vendor/petgraph/src/graphmap.rs:580:16
    |
580 |     edges: &'a IndexMap<(N, N), E>,
    |                ^^^^^^^^ ------  - supplied 2 generic arguments
    |                |
    |                expected 3 generic arguments
    |
help: add missing generic argument
    |
580 |     edges: &'a IndexMap<(N, N), E, S>,
    |                                  +++ 
