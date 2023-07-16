rust
pub enum ObligationCauseCode<'tcx> {
    ///  An obligation incurred from treating a higher-ranked trait situated at the specified Span.
    HigherRankedRegion(Span) // do we need to keep a nested ObligationCauseCode here?
}
// code_to_error can now check the ObligationCauseCode if it sees a `ReStatic` bound
// and emit a custom error with both the obligation's Span (`&'b i32`) as well as the HRTB (`for<'a> T: 'a {}`).
