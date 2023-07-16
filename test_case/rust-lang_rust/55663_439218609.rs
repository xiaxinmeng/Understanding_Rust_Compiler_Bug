
[00:20:51] error[E0507]: cannot move out of an `Rc`
[00:20:51]   --> librustc_lint/unused.rs:71:43
[00:20:51]    |
[00:20:51] 71 |                     for (predicate, _) in cx.tcx.predicates_of(def).predicates {
[00:20:51]    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of an `Rc`
[00:20:52] error: aborting due to previous error
