rust
// HACK(eddyb) needed to deal with `Offset` vs `Size` type differences.
let layout_extent = Offset::ZERO..Offset::ZERO+layout.size;

assert_eq!(
    scalar_leaf_within(layout, layout_extent),
    Ok(ScalarLeaf {
        extent: layout_extent,
        primitive: scalar.primitive,
    }),
);
