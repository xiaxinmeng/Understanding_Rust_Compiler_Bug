
error[E0191]: the value of the associated type `Item` (from trait `Iterator`) must be specified
   --> library/core/src/iter/traits/iterator.rs:16:35
    |
16  | fn _assert_is_object_safe(_: &dyn Iterator<Item = ()>) {}
    |                                   ^^^^^^^^^^^^^^^^^^^ help: specify the associated type: `Iterator<Item = (), Item = Type>`
...
100 |     type Item;
    |     ---------- `Item` defined here
