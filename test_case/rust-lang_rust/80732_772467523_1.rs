
$ cargo +c963187c6f959417cbb13a33e9eaea4607696fc4 build
   Compiling salsa-min v0.1.0 (/home/grayshade/salsa-min)
error[E0221]: ambiguous associated type `Group` in bounds of `Q`
  --> src/main.rs:9:53
   |
9  |         <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<Q::Group>>::group_storage(db);
   |                                                     ^^^^^^^^ ambiguous associated type `Group`
...
30 |     type Group;
   |     -----------
   |     |
   |     ambiguous `Group` from `for<'d> QueryDb<'d>`
   |     ambiguous `Group` from `for<'d> QueryDb<'d>`
   |
help: use fully qualified syntax to disambiguate
   |
9  |         <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use fully qualified syntax to disambiguate
   |
9  |         <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0221`.
error: could not compile `salsa-min`

To learn more, run the command again with --verbose.

$ cargo +stable check
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
