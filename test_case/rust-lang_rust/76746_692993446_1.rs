rust
self.attrs.iter().fold(self.span, |acc, attr| acc.to(attr.span))
.fold(Fingerprint::ZERO, |a, b| a.combine_commutative(b));
exprs.iter().rev().fold(succ, |succ, expr| self.propagate_through_expr(&expr, succ))
