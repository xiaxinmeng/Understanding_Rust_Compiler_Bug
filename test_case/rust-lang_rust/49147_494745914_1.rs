rust
ValueSource::Rvalue(&Rvalue::Repeat(operand, _)) => {
    let candidate = Candidate::Repeat(location);
    let not_promotable =
        IsNotPromotable::in_operand(self, operand) ||
        IsNotImplicitlyPromotable::in_operand(self, operand);
    if !not_promotable {
        debug!("qualify_consts: promotion candidate: {:?}",  candidate);
        self.promotion_candidates.push(candidate);
    }
}
