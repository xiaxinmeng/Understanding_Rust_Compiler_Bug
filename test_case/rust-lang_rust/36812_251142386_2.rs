 rust
// This gets called when the visitor encounters a struct field
fn visit_struct_field(&mut self, s: &'tcx StructField) {
    // We record that we saw a struct field
    SawStructField.hash(self.st);
    // We hash the span of the field
    hash_span!(self, s.span);
    // We hash the attributes of the field
    hash_attrs!(self, &s.attrs);
    // Now we recurse further into the StructField structure
    // where the visitor will encounter the name of the field
    // (and hash it via visit_name()) and its type (and hash it
    // via visit_ty())
    visit::walk_struct_field(self, s)
}
