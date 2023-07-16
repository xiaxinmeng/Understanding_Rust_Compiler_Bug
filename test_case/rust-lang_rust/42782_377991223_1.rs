
trait IteratorWithBreakingForEach : Iterator {
    fn breaking_for_each<F, BI>(self, f: F) -> BI
        where F: FnMut(Self::Item) -> BI,
              BI: BreakIndicator,
    ;
}

impl<I> IteratorWithBreakingForEach for I where I: Iterator {
    fn breaking_for_each<F, BI>(self, mut f: F) -> BI
        where F: FnMut(Self::Item) -> BI,
              BI: BreakIndicator,
    {
        for item in self {
            let break_indicator = f(item);
            if break_indicator.is_break() {
                return break_indicator;
            }
        }
        BI::final_continue()
    }
}
