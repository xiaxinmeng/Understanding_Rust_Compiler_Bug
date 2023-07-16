
pub enum RangeInclusive<Idx> {
    Empty {
        at: Idx,
    },
    NonEmpty {
        start: Idx,
        end: Idx,
    },
}
