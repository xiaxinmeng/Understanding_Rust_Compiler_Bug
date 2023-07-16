rust
match &args[0] {
    Operand::Copy(place) | Operand::Move(place) => // how to get the type of `Place` here?
    Operand::Constant(constant) => // compare constant.ty, but against what?
}
