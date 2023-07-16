
error: layout error: Unknown(<[E] as std::borrow::ToOwned>::Owned)
 --> src/lib.rs:6:1
  |
6 | struct Edges<'a, E>(Cow<'a, [E]>) where [E]: ToOwned;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
