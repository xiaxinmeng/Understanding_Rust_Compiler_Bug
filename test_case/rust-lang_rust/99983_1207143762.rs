rust
// HACK(eddyb) used below to disambiguate between the two uses of `Size`,
// but we should probably have this as a completely separate type, with:
// `Size`: measured in bytes and/or bits; Add+Mul ops (Sub not needed)
// `Offset`: measured in bytes (no "bit offsets"); Add+Sub ops (Mul not needed)
type Offset = Size;

// HACK(eddyb) necessary method sadly missing from `std::ops::Range`.
impl<T> Range<T> {
    fn includes(&self, other: Range<T>) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

struct ScalarLeaf {
    /// Where in the parent layout this leaf is.
    ///
    /// Must be the right size, i.e. `extent.end - extent.start == primitive`.
    //
    // FIXME(eddyb) does the above condition make `extent.end` redundant?
    // (move to a `fn extent(&self, cx: ...) -> Range<Offset>` helper method?)
    extent: Range<Offset>,

    // FIXME(eddyb) instead of ignoring validity ranges, should they be merged
    // across e.g. `enum` variants?
    primitive: Primitive,
}

struct ScalarLeafWithinError {
    /// No leaves found in `search_extent`.
    OnlyPadding,

    /// A leaf intersects `search_extent` without being contained in it,
    /// i.e. the leaf is "straddling" `search_extent`'s start or end.
    PartialIntersection(ScalarLeaf),

    /// Two leaves found within `search_extent` that differ in their `extent`,
    /// `primitive`, or both.
    Mismatch(ScalarLeaf, ScalarLeaf),
}

/// Find the unique `Scalar` (but ignoring validity ranges, so... `Primitive`¹?)
/// leaf field wholly contained within `search_extent` in `layout`.
/// 
/// An `Ok(leaf)` result indicates a leaf field that is:
/// * "wholly contained within": `search_extent.includes(leaf.extent)`
/// * "unique": `layout` always contains (across all `enum`/`union` cases):
///   * `Scalar(leaf.primitive)` exactly filling `leaf.extent`
///     * some `enum`/`union` cases may contain padding at `leaf.extent` instead,
///       but all other `enum`/`union` cases must agree on the same `leaf`
///     * `Scalar` validity might vary, but the `Primitive` must match exactly
///   * only padding, in `search_extent` "around" `leaf.extent`,
///     i.e. neither of these extents contain any leaves of their own:
///     * `search_extent.start..leaf.extent.start` (before `leaf`)
///     * `leaf.extent.end..search_extent.end` (after `leaf`)
/// 
/// ¹ I wish `Primitive` was called `Scalar` but what would we call `Scalar`?
///   (maybe `Primitive` should be renamed to `ScalarType`/`ScalarKind`?)
fn scalar_leaf_within(
    layout: &TyAndLayout<'tcx>,
    search_extent: Range<Offset>,
) -> Result<ScalarLeaf, ScalarLeafWithinError> {
    fn try_for_each_leaf_intersecting<E>(
        base_offset: Offset,
        layout: &TyAndLayout<'tcx>,
        filter_extent: Range<Offset>,
        each: &mut impl FnMut(ScalarLeaf) -> Result<(), E>
    ) -> Result<(), E> {
        for (i, field_offset) in layout.fields.enumerate() {
            if field_offset < filter_extent.end {
                let field = layout.field(i);
                if filter_extent.start < field_offset + field.size {
                    try_for_each_leaf_intersecting(
                        base_offset + field_offset,
                        field,
                        filter_extent,
                        each,
                    )?;
                }
            }
        }
        for v in 0..layout.variants.len() {
            try_for_each_leaf_intersecting(base_offset, layout.for_variant(v), filter_extent, each)?;
        }

        if layout.fields.is_empty() && !layout.is_zst() {
            match layout.abi {
                Abi::Scalar(scalar) => each(ScalarLeaf {
                    extent: base_offset..base_offset+scalar.size(),
                    primitive: scalar.primitive,
                })?,

                // FIXME(eddyb) can we guarantee all non-ZST leaves are `Scalar`?
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    let mut found = None;
    try_for_each_leaf_intersecting(
        Offset::ZERO,
        layout,
        search_extent,
        |candidate| match found {
            _ if !search_extent.includes(&candidate.extent) => {
                Err(ScalarLeafError::PartialIntersection(candidate))
            }
            Some(previous) if candidate != previous => {
                Err(ScalarLeafError::Mismatch(previous, candidate))
            }
            _ => {
                found = Some(candidate);
                Ok(())
            }
        }
    )?;
    Ok(found.ok_or(ScalarLeafWithinError::OnlyPadding))
}
