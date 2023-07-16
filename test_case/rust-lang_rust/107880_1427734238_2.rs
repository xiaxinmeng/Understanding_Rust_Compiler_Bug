rust
/// The return type of [`sum(expr)`](crate::dsl::sum())
pub type sum<Expr> = super::aggregate_folding::sum::HelperType<SqlTypeOf<Expr>, Expr>;
