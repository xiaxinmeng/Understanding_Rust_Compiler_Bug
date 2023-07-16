
match Enum::Unit {
    Enum::Unit => {}
    Enum::Struct { x: true, .. } => {}
    Enum::Struct { x: false, .. } => {}
}
