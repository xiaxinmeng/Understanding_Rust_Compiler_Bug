 rust
fn substitute_dummy(self, other: Span) -> Span {
    if *self == DUMMY_SP { other } else { *self }
}
