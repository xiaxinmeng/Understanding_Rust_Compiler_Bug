rust
struct Statement {
    // Does not contain any further `Place`s, `Operand`s, or other "runtime" values
    kind: StatementKind,
    places: Vec<Place>,
    operands: Vec<Operand>,
    // source_info, etc.
}
