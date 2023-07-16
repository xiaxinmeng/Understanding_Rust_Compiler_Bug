
error[E0221]: ambiguous associated type `Group` in bounds of `Q`
   --> src/interned.rs:466:57
    |
466 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<Q::Group>>::group_storage(db);
    |                                                         ^^^^^^^^ ambiguous associated type `Group`
    | 
   ::: src/lib.rs:431:5
    |
431 |     type Group: plumbing::QueryGroup<GroupStorage = Self::GroupStorage>;
    |     --------------------------------------------------------------------
    |     |
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |
help: use fully qualified syntax to disambiguate
    |
466 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use fully qualified syntax to disambiguate
    |
466 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0221]: ambiguous associated type `Group` in bounds of `Q`
   --> src/interned.rs:478:57
    |
478 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<Q::Group>>::group_storage(db);
    |                                                         ^^^^^^^^ ambiguous associated type `Group`
    | 
   ::: src/lib.rs:431:5
    |
431 |     type Group: plumbing::QueryGroup<GroupStorage = Self::GroupStorage>;
    |     --------------------------------------------------------------------
    |     |
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |
help: use fully qualified syntax to disambiguate
    |
478 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use fully qualified syntax to disambiguate
    |
478 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0221]: ambiguous associated type `Group` in bounds of `Q`
   --> src/interned.rs:490:57
    |
490 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<Q::Group>>::group_storage(db);
    |                                                         ^^^^^^^^ ambiguous associated type `Group`
    | 
   ::: src/lib.rs:431:5
    |
431 |     type Group: plumbing::QueryGroup<GroupStorage = Self::GroupStorage>;
    |     --------------------------------------------------------------------
    |     |
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |
help: use fully qualified syntax to disambiguate
    |
490 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use fully qualified syntax to disambiguate
    |
490 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0221]: ambiguous associated type `Group` in bounds of `Q`
   --> src/interned.rs:512:57
    |
512 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<Q::Group>>::group_storage(db);
    |                                                         ^^^^^^^^ ambiguous associated type `Group`
    | 
   ::: src/lib.rs:431:5
    |
431 |     type Group: plumbing::QueryGroup<GroupStorage = Self::GroupStorage>;
    |     --------------------------------------------------------------------
    |     |
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |     ambiguous `Group` from `for<'d> QueryDb<'d>`
    |
help: use fully qualified syntax to disambiguate
    |
512 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use fully qualified syntax to disambiguate
    |
512 |             <<Q as QueryDb<'_>>::DynDb as HasQueryGroup<<Q as for<'d> QueryDb<'d>>::Group>>::group_storage(db);
    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0221`.
error: could not compile `salsa`
