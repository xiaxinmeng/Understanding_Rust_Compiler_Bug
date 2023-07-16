rust
pub fn range_count<A: Step>(range: std::ops::Range<A>) -> usize {
    if let Some(steps) = Step::steps_between(&range.start, &range.end) {
        steps
    } else {
        range.fold(0, |x, _| x + 1)
    }
}
