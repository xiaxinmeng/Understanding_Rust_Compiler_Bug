rust
// Avoid creating a temporary
ExprKind::VarRef { .. }
| ExprKind::UpvarRef { .. }
| ExprKind::PlaceTypeAscription { .. }
| ExprKind::ValueTypeAscription { .. } => {
    debug_assert!(Category::of(&expr.kind) == Some(Category::Place));

    let place = unpack!(block = this.as_place(block, expr));
    let rvalue = Rvalue::Use(this.consume_by_copy_or_move(place));

    if this.generator_kind.is_some() {
        if let ExprKind::UpvarRef { .. } = &expr.kind {
            let local = destination.local;
            assert!(destination.projection.is_empty());
            match place.projection[0] {
                PlaceElem::Field(field, _) => {
                    println!("Storing upvar {:?} into {:?}", field.as_usize(), local);
                    // Store into Builder and then propagate to generator MIR transform
                }
                _ => panic!("Unexpected upvar field")
            }
        }
    }

    this.cfg.push_assign(block, source_info, destination, rvalue);
    block.unit()
}
