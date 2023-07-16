
Mar 25 20:42:41.973 INFO kablam! error[E0282]: type annotations needed
Mar 25 20:42:41.973 INFO kablam!     --> src/screen.rs:2119:49
Mar 25 20:42:41.973 INFO kablam!      |
Mar 25 20:42:41.973 INFO kablam! 2118 |                 let children_new = children.into_iter().collect();
Mar 25 20:42:41.973 INFO kablam!      |                     ------------ consider giving `children_new` a type
Mar 25 20:42:41.973 INFO kablam! 2119 |                 let intersection = children_acc.intersection(&children_new);
Mar 25 20:42:41.974 INFO kablam!      |                                                 ^^^^^^^^^^^^ cannot infer type for `U`
