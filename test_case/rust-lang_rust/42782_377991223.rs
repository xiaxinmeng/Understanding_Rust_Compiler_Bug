
trait BreakIndicator {
    fn is_break(&self) -> bool;
    fn final_continue() -> Self;
}

impl BreakIndicator for () {
    // always continue
    fn is_break(&self) -> bool { false }
    fn final_continue() -> () {}
}

impl BreakIndicator for bool {
    // true means break; false means continue
    fn is_break(&self) -> bool { *self }
    fn final_continue() -> bool { false }
}

impl<T> BreakIndicator for Option<T> {
    // Some(v) means "break wich value v"; None means continue
    fn is_break(&self) -> bool { self.is_some() }
    fn final_continue() -> Option<T> { None }
}
