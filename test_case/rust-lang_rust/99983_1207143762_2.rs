rust
// HACK(eddyb) needed to deal with `Offset` vs `Size` type differences.
let layout_extent = Offset::ZERO..Offset::ZERO+layout.size;
let a_extent = Offset::ZERO..Offset::ZERO+a.size;
// FIXME(eddyb) there should be a nicer way to make `start..start+len` ranges.
let b_extent = a_extent.end.align_to(b.align)..;
let b_extent = b_extent.start..b_extent.start+b.size;

assert_eq!(
    scalar_leaf_within(layout, layout_extent.start..b_extent.start),
    Ok(ScalarLeaf {
        extent: a_extent,
        primitive: a.primitive,
    }),
);
assert_eq!(
    scalar_leaf_within(layout, b_extent.start..layout_extent.end),
    Ok(ScalarLeaf {
        extent: b_extent,
        primitive: b.primitive,
    }),
);
