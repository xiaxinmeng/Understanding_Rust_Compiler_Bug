rust
type LeafFolder/Visitor: TypeFolder/Visitor<'tcx> = !;
fn leaf_folder/visitor(&mut self) -> Option<&mut Self::LeafFolder/Visitor> {
    None
}
