
error: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions.
  --> src/graph.rs:77:6
   |
77 | impl<Ix: IndexType = DefIndex> NodeIndex<Ix>
   |      ^^
