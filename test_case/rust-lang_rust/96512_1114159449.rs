diff
- ty::Adt(adt_def, substs) if !adt_def.is_enum() => {
+ ty::Adt(adt_def, substs) if adt_def.variants().len() == 1 => {
