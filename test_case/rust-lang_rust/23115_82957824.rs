 rust
// from Parser::parse_more_binops
let lhs_span = lhs.span;
let rhs_span = rhs.span;
let binary = self.mk_binary(codemap::respan(cur_op_span, cur_op), lhs, rhs);
let bin = self.mk_expr(lhs_span.lo, rhs_span.hi, binary);
