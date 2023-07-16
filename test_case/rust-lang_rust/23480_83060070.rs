
macro_rules! add1 { ($val:expr) => ( { 1 + $val } ) }
                                       ^^^^^^^^
// the span of the (+) AST node lies within the macro-definition,
// regardless where $val comes from
