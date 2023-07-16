plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find function `from_def_id` in this scope
   --> src/librustdoc/json/conversions.rs:389:25
    |
174 | crate fn from_item_id(did: ItemId) -> Id {
    | ---------------------------------------- similarly named function `from_item_id` defined here
...
389 |                     id: from_def_id(id.into()),
    |                         ^^^^^^^^^^^ help: a function with a similar name exists: `from_item_id`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustdoc`
