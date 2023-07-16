rust
if self.check(&token::Not) {
    self.span_err(self.span, "Negative trait bounds are not supported");
}
