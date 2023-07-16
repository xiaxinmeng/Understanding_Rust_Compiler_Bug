 rust
pub enum TypeVariableOrigin {
    TypeParameterDefinition(Span, ast::Name),
    TransformedUpvar(Span, ast::Name),
    AdjustNeverToAny(Span, ast::Name),
    DivergingStatement(Span, ast::Name),
    DivergingBlockExpression(Span, ast::Name),
}
