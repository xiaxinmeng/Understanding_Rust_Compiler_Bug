 rust
impl PropertyDeclaration {
    fn discriminant_value(&self) -> usize {
        match *self {
            PropertyDeclaration::AlignContent(..) => 0,
            PropertyDeclaration::AlignItems(..) => 1,
            PropertyDeclaration::AlignSelf(..) => 2,
            // ...
            PropertyDeclaration::ZIndex(..) => 147,
        }
    }
}
